use shared::protocol::{Command, Response};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, tcp::OwnedWriteHalf},
};
use tracing::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    info!("Server listening on 127.0.0.1:8080");

    loop {
        let (sock, addr) = listener.accept().await?;
        info!("New connection from {}", addr);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(sock).await {
                error!("Connection error: {:?}", e);
            }
        });
    }
}

async fn handle_connection(sock: tokio::net::TcpStream) -> anyhow::Result<()> {
    let (r, mut w) = sock.into_split();
    let mut reader = BufReader::new(r);

    let mut line = String::new();
    reader.read_line(&mut line).await?;

    if line.is_empty() {
        return Ok(());
    }

    let command: Command = match serde_json::from_str(&line) {
        Ok(cmd) => cmd,
        Err(_) => {
            let err = Response::Error {
                message: "Invalid JSON".into(),
            };
            send_response(&mut w, err).await?;
            return Ok(());
        }
    };

    info!("Received command {:?}", command);

    // responses
    match command {
        Command::Add { .. } => {
            let res = Response::Ok;
            send_response(&mut w, res).await?;
        }
        Command::List { .. } => {
            let res = Response::Services {
                services: Vec::new(),
            };
            send_response(&mut w, res).await?;
        }
        Command::Fetch { service, .. } => {
            let res = Response::Error {
                message: format!("No secret for {}", service),
            };
            send_response(&mut w, res).await?;
        }
    }

    Ok(())
}

async fn send_response(w: &mut OwnedWriteHalf, res: Response) -> anyhow::Result<()> {
    let json = serde_json::to_string(&res)?;
    w.write_all(json.as_bytes()).await?;
    w.write_all(b"\n").await?;
    w.flush().await?;

    Ok(())
}
