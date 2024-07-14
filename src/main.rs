use tokio::net::{TcpListener, TcpStream};
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use log::{info, LevelFilter};
use env_logger::Builder;

#[tokio::main]
async fn main() {
    setup_env();
    setup_logging();
        
    let url = env::var("BACKEND_URL").unwrap();

    let listener = TcpListener::bind(&url).await.unwrap();
    info!("Listening on: {}", url);
    
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr));
    }
}

fn setup_env() {
    dotenv().ok();
}

fn setup_logging() {
    let mut builder = Builder::new();
    
    let debug = env::var("DEBUG").unwrap_or_default() == "true";
    let level = if debug { LevelFilter::Info } else { LevelFilter::Error };
    builder.filter(None, level);
    
    builder.init();
}

async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr){
    info!("Handling connection!")
}