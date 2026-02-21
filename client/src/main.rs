use clap::{Parser, Subcommand};
use shared::protocol::Command;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        username: String,
        password: String,
        service: String,
        secret: String,
    },
    List {
        username: String,
        password: String,
    },
    Fetch {
        username: String,
        password: String,
        service: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let command = match cli.command {
        Commands::Add {
            username,
            password,
            service,
            secret,
        } => Command::Add {
            username,
            password,
            service,
            secret,
        },
        Commands::List { username, password } => Command::List { username, password },
        Commands::Fetch {
            username,
            password,
            service,
        } => Command::Fetch {
            username,
            password,
            service,
        },
    };

    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let (read_half, mut write_half) = stream.into_split();

    let json = serde_json::to_string(&command)?;

    write_half.write_all(json.as_bytes()).await?;
    write_half.write_all(b"\n").await?;
    write_half.flush().await?;

    let mut reader = BufReader::new(read_half);
    let mut response = String::new();
    reader.read_line(&mut response).await?;

    println!("Response from client: {}", response.trim());

    Ok(())
}
