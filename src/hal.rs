use crate::printing::Instruction;
use crate::tcp_talker::run_tcp_client;
use anyhow::Context;
use bytes::Bytes;
use log::{debug, info, warn};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::Notify;

pub struct Hal {
    receiver: UnboundedReceiver<Instruction>,
    notifier: Arc<Notify>,
    tx: Sender<Bytes>,
}

impl Hal {
    pub fn new(receiver: UnboundedReceiver<Instruction>) -> Self {
        let notifier = Arc::new(Notify::new());
        let (tx, rx) = broadcast::channel(1024);
        Hal {
            receiver,
            tx,
            notifier,
        }
    }

    pub async fn run(&mut self) {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3500);
        run_tcp_client(addr, self.tx.clone(), self.notifier.clone());

        self.prepare();

        debug!("runner started");

        self.notifier.notified().await;
        let _ = self.elaborate_messages().await;

        self.graceful_shutdown();
    }

    pub async fn elaborate_messages(&mut self) -> anyhow::Result<()> {
        while let Some(item) = self.receiver.recv().await {
            debug!("received message: {:?}", &item);
            match item {
                Instruction::Halt => break,
                Instruction::SendBytes(details) => self.transmit_bytes(&details.cmd)?,
                Instruction::Shutdown => {
                    self.shutdown();
                    break;
                }
            }
        }
        Ok(())
    }

    fn graceful_shutdown(&mut self) {
        warn!("graceful shutdown");
    }

    pub fn transmit_bytes(&mut self, bytes: &[u8]) -> anyhow::Result<()> {
        self.tx
            .send(Bytes::from(bytes.to_vec()))
            .map(|_| ())
            .context("cannot transmit bytes")
    }

    pub fn prepare(&mut self) {
        self.go_online();
        self.start_accepting_commands();
    }

    fn go_offline(&mut self) {
        info!("go off-line");
        self.transmit_bytes(&[0xA0, 0x00]);
    }

    fn go_online(&mut self) {
        info!("go on-line");
        self.transmit_bytes(&[0xA1, 0x00]);
    }

    fn start_accepting_commands(&mut self) {
        info!("start accepting printing commands");
        self.transmit_bytes(&[0xA2, 0x00]);
        info!("machine is now accepting the commands");
    }

    fn stop_accepting_commands(&mut self) {
        info!("stop accepting printing commands");
        self.transmit_bytes(&[0xA3, 0x00]);
    }

    pub fn shutdown(&mut self) {
        self.stop_accepting_commands();
        self.go_offline();
    }
}
