use thiserror::Error;
use tokio::{
    net::{TcpListener, TcpStream},
    try_join,
};

#[derive(Error, Debug)]
pub enum ProxyError {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = "0.0.0.0";
    let port = 3000;
    let addr = format!("{host}:{port}");
    println!("Server is listening at http://{addr}");
    let server = TcpListener::bind(addr).await?;

    while let Ok((client, _)) = server.accept().await {
        tokio::spawn(async move {
            if let Err(err) = handle_connection(client).await {
                println!("Something went wrong {err}");
            }
        });
    }
    Ok(())
}

async fn handle_connection(mut client_conn: TcpStream) -> Result<(), ProxyError> {
    let mut server_conn = TcpStream::connect("192.168.1.104:80").await?;

    let (mut client_read, mut client_write) = client_conn.split();
    let (mut server_read, mut server_write) = server_conn.split();

    let handle_server_read = async { tokio::io::copy(&mut server_read, &mut client_write).await };
    let handle_client_read = async { tokio::io::copy(&mut client_read, &mut server_write).await };

    try_join!(handle_server_read, handle_client_read)?;
    Ok(())
}
