use clap::Parser;
use anyhow::Result;
mod config;
mod smtp;

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

fn main() -> Result<()>{
    let opts: Opts = Opts::parse();

    let config = config::EmailConfig::from_env()?;

    let smtp_client = smtp::SmtpClient::new(&config)?;
    smtp_client.send_test_email(&opts.email)?;

    println!("ppnp (file: {}, email: {})", opts.file, opts.email);
    Ok(())
}
