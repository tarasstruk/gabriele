use crate::printing::Instruction;
use crate::tcp_talker::run_tcp_client;
use anyhow::Context;
use bytes::Bytes;
use log::debug;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::sync::broadcast::{self, Sender};
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
        let (tx, _rx) = broadcast::channel(1024);
        Hal {
            receiver,
            tx,
            notifier,
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3500);
        run_tcp_client(addr, self.tx.clone(), self.notifier.clone());

        self.notifier.notified().await;
        self.prepare()?;
        debug!("runner is started successfully");
        self.elaborate_messages().await?;
        self.shutdown()
    }

    pub async fn elaborate_messages(&mut self) -> anyhow::Result<()> {
        while let Some(item) = self.receiver.recv().await {
            debug!("received message: {:?}", &item);
            match item {
                Instruction::Halt => break,
                Instruction::SendBytes(details) => self.transmit_bytes(&details.cmd)?,
                Instruction::Shutdown => {
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn transmit_bytes(&self, bytes: &[u8]) -> anyhow::Result<()> {
        self.tx
            .send(Bytes::from(bytes.to_vec()))
            .map(|_| ())
            .context("cannot transmit bytes")
    }

    pub fn prepare(&self) -> anyhow::Result<()> {
        self.go_online()?;
        self.start_accepting_commands()
    }

    fn go_offline(&self) -> anyhow::Result<()> {
        self.transmit_bytes(&[0xA0, 0x00])
    }

    fn go_online(&self) -> anyhow::Result<()> {
        self.transmit_bytes(&[0xA1, 0x00])
    }

    fn start_accepting_commands(&self) -> anyhow::Result<()> {
        self.transmit_bytes(&[0xA2, 0x00])
    }

    fn stop_accepting_commands(&self) -> anyhow::Result<()> {
        self.transmit_bytes(&[0xA3, 0x00])
    }

    pub fn shutdown(&self) -> anyhow::Result<()> {
        self.stop_accepting_commands()?;
        self.go_offline()
    }
}
