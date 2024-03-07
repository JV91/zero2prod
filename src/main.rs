use zero2prod::run;
use std::net::TcpListener;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    run(TcpListener)?.await
}