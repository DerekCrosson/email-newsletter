use email_newsletter::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = 8000;
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(address).expect("Failed to bind port");

    run(listener)?.await
}
