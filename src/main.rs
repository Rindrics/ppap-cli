use clap::Parser;
mod config;

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

fn main() {
    let opts: Opts = Opts::parse();
    println!("ppnp (file: {}, email: {})", opts.file, opts.email);
}
