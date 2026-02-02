mod email;
mod zip;

use clap::Parser;
use anyhow::Result;
use std::io::Write;
use email::{
    config::SendGridConfig,
    sender::EmailSender,
    sendgrid::SendGridRestSender,
};

#[derive(Parser, Debug)]
#[command(name = "ppap")]
#[command(author = "Rindrics")]
#[command(version = "0.1.0")]
#[command(about = "CLI tool that uses traditional Japanese file sharing protocol")]
struct Opts {
    #[arg(help = "Target file to be sent")]
    file: String,

    #[arg(help = "Email address of the recipient")]
    email: String,

    #[arg(short = 's', long = "secure",
          help = "Enhance security by sending an incorrect password (Good luck explaining this to your recipient)")]
    secure: bool,

    #[arg(short = 'a', long = "after", value_name = "HOURS",
          help = "Delay password delivery by specified hours (Because waiting makes everything more secure)")]
    after: Option<u64>,
}

async fn async_main() -> Result<()> {
    let opts: Opts = Opts::parse();

    // Step 1: Compress file and get password
    println!("Compressing file: {}", opts.file);
    let (zip_path, password) = zip::compress_file(&opts.file)?;
    println!("File compressed successfully: {}", zip_path);

    // Step 2: Initialize SendGrid
    let config = SendGridConfig::from_env()?;
    let sender = SendGridRestSender::new(&config);

    // Step 3: Send email #1 with ZIP attachment
    println!("\nSending email #1: ZIP file attachment");
    sender.send_email_with_attachment(
        &opts.email,
        "Secure File Transfer - Encrypted Document",
        "Please find the attached password-protected file.\n\nThe password will be sent in a separate email for security purposes.",
        &zip_path,
    ).await?;
    println!("Email #1 sent successfully!");

    // Step 4: Determine which password to send
    let password_to_send = if opts.secure {
        println!("\n[SECURE MODE ACTIVATED]");
        println!("Generating incorrect password for maximum security...");
        get_password_to_send(&password, true)
    } else {
        password.clone()
    };

    // Step 5: Handle delayed sending if requested
    if let Some(hours) = opts.after {
        println!("\nPassword email will be sent in {} hours.", hours);
        println!("Please keep this terminal open. Press Ctrl+C to cancel.");
        println!("(File email has already been sent)");

        let total_seconds = hours * 3600;
        for elapsed in (0..total_seconds).step_by(60) {
            let remaining = total_seconds - elapsed;
            print!("\rWaiting... ({:02}:{:02}:{:02} remaining)  ",
                   remaining/3600, (remaining%3600)/60, remaining%60);
            let _ = std::io::stdout().flush();
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        }
        println!("\n\nDelay complete! Sending password email now...");
    }

    // Step 6: Send email #2 with password
    println!("\nSending email #2: Password");
    let password_email_body = format!(
        "Password for the encrypted file: {}\n\nPlease use this password to open the ZIP file sent in the previous email.",
        password_to_send
    );

    sender.send_email(
        &opts.email,
        "Secure File Transfer - Password",
        &password_email_body,
    ).await?;
    println!("Email #2 sent successfully!");

    // Step 7: Cleanup
    println!("\nCleaning up temporary files...");
    zip::cleanup_temp_file(&zip_path)?;

    println!("\n=== PPAP Protocol Complete ===");
    println!("File sent successfully to: {}", opts.email);
    if opts.secure {
        println!("[WARNING] Secure mode was enabled. The recipient received an INCORRECT password.");
        println!("Real password (for your records): {}", password);
    }

    Ok(())
}

fn get_password_to_send(real_password: &str, secure_mode: bool) -> String {
    if secure_mode {
        use rand::{thread_rng, Rng};
        use rand::distributions::Alphanumeric;
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(real_password.len())
            .map(char::from)
            .collect()
    } else {
        real_password.to_string()
    }
}

#[tokio::main]
async fn main() -> Result<()>{
    async_main().await
}
