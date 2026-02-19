use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'u', long = "username")]
    username: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    println!("Hello {}", cli.username);
}
