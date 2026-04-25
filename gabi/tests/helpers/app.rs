use anyhow::Result;
use gabriele::machine::Machine;
use log::warn;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio::task::JoinHandle;

use gabi::{Hal, SenderWrapper};
use gabriele::symbol::Symbol;

pub struct TestApp {
    pub machine: Machine<SenderWrapper, &'static [Symbol]>,
    machine_handle: JoinHandle<Result<()>>,
    pub rx: UnboundedReceiver<u8>,
    server_handle: JoinHandle<()>,
}
impl TestApp {
    pub async fn run(port: u16) -> TestApp {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        let (sender, receiver) = unbounded_channel();
        let db: &'static [Symbol] = &gabriele::wheels::standard::SYMBOLS;
        let machine = Machine::new(SenderWrapper(sender), db);

        let mut hal = Hal::new(receiver, addr.clone());
        let machine_handle = tokio::spawn(async move { hal.run().await });
        let (rx, server_handle) = start_test_server(addr);
        Self {
            machine,
            machine_handle,
            rx,
            server_handle,
        }
    }

    pub async fn halt(&mut self) {
        self.machine.shutdown().await;
    }

    pub async fn teardown(mut self) {
        self.machine.shutdown().await;
        let _ = self.machine_handle.await.unwrap();
        self.server_handle.abort();
        let _ = self.server_handle.await;
    }
}

fn start_test_server(addr: SocketAddr) -> (UnboundedReceiver<u8>, JoinHandle<()>) {
    let (sender, receiver) = unbounded_channel();
    let handle = tokio::spawn(async move {
        let listener = TcpListener::bind(addr).await.unwrap();
        let (mut socket, _) = listener
            .accept()
            .await
            .expect("accepting tcp connection failed");
        let (mut reader, mut writer) = socket.split();
        loop {
            if let Ok(byte) = reader.read_u8().await {
                if writer.write_u8(byte).await.is_err() {
                    break;
                }
                if sender.send(byte).is_err() {
                    break;
                }
            } else {
                break;
            }
        }
        drop(sender);
        warn!("SENDER DROPPED");
    });
    (receiver, handle)
}
