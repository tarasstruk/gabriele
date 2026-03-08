use anyhow::Result;
use clap::Parser;
use env_logger::{Builder, Target};
use log::warn;
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::fs::File;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// IP address for the listener
    #[arg(long, default_value_t = Ipv4Addr::from([127,0,0,1]))]
    ip: Ipv4Addr,

    #[arg(long, default_value_t = 1234)]
    /// Port number
    port: u16,

    /// Output file name
    #[arg(long, default_value_t = String::from("output.bin"))]
    path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();
    let args = Args::parse();
    warn!("Args: {:?}", args);

    let mut file = File::create(args.path).await?;

    let socket_addr = SocketAddrV4::new(args.ip, args.port);
    let listener = TcpListener::bind(socket_addr).await?;
    warn!("listening on localhost:1234");
    let (socket, _) = listener.accept().await?;
    let (mut rd, mut wr) = io::split(socket);

    loop {
        let byte = match rd.read_u8().await {
            Ok(b) => b,
            Err(_) => {
                warn!("reader got EOF");
                break;
            }
        };

        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        wr.write_u8(byte).await?;
        file.write_u8(byte).await?;
    }

    // Ensure data is pushed to the disk
    file.flush().await?;

    Ok(())
}
