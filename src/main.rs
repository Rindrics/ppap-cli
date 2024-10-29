mod email;
mod zip;

use clap::Parser;
use anyhow::Result;
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
}

async fn async_main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let zip_path = zip::compress_file(&opts.file)?;

    let config = SendGridConfig::from_env()?;

    let sender = SendGridRestSender::new(&config);

    sender.send_email(
        &opts.email,
        "File Transfer",
        "Please find the attached file.",
    ).await?;

    println!("File sent successfully to: {}", opts.email);

    zip::cleanup_temp_file(&zip_path)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()>{
    async_main().await
}
