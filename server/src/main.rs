use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (mut sock, addr) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            println!("New connection from {}", addr);
            loop {
                let n = match sock.read(&mut buf).await {
                    Ok(0) => return, // socket closed
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                // write the data back
                if let Err(e) = sock.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
