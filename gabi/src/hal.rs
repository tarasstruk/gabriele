use anyhow::{bail, Context};
use bytes::Bytes;
use gabriele::printing::Instruction;
use log::debug;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tcp_client::run_tcp_client;
use tokio::sync::broadcast::{self, Sender};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

pub struct Hal {
    receiver: UnboundedReceiver<Instruction>,
    notifier: Arc<Notify>,
    tx: Option<Sender<Bytes>>,
    c_token: CancellationToken,
    socket_addr: SocketAddr,
}

impl Hal {
    pub fn new(receiver: UnboundedReceiver<Instruction>, socket_addr: SocketAddr) -> Self {
        let notifier = Arc::new(Notify::new());

        let c_token = CancellationToken::new();
        Hal {
            receiver,
            tx: None,
            notifier,
            c_token,
            socket_addr,
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        let (tx, rx) = broadcast::channel::<Bytes>(1024);
        self.tx.replace(tx);

        let token = self.c_token.clone();
        let handle = run_tcp_client(self.socket_addr, rx, self.notifier.clone(), token);

        self.notifier.notified().await;
        // self.prepare()?;
        debug!("runner is started successfully");
        self.elaborate_messages().await?;
        debug!("sender channel is disconnected");
        self.c_token.cancel();
        // self.shutdown()?;
        tokio::time::sleep(Duration::from_secs(1)).await;

        let _ = tokio::join!(handle);
        Ok(())
    }

    pub async fn elaborate_messages(&mut self) -> anyhow::Result<()> {
        while let Some(item) = self.receiver.recv().await {
            debug!("received message: {:?}", &item);
            match item {
                Instruction::SendBytes(details) => self.transmit_bytes(details)?,
                Instruction::Halt => break,
            }
        }
        // drop Sender
        let _ = self.tx.take();
        Ok(())
    }

    pub fn transmit_bytes(&self, word: u16) -> anyhow::Result<()> {
        if let Some(ref tx) = self.tx {
            tx.send(Bytes::copy_from_slice(&word.to_be_bytes()))
                .map(|_| ())
                .context("cannot transmit bytes")
        } else {
            bail!("cannot transmit bytes, channel is disconnected")
        }
    }

    // pub fn prepare(&self) -> anyhow::Result<()> {
    //     self.go_online()?;
    //     self.start_accepting_commands()
    // }

    // fn go_offline(&self) -> anyhow::Result<()> {
    //     self.transmit_bytes(&[0xA0, 0x00])
    // }

    // fn go_online(&self) -> anyhow::Result<()> {
    //     self.transmit_bytes(&[0xA1, 0x00])
    // }
    //
    // fn start_accepting_commands(&self) -> anyhow::Result<()> {
    //     self.transmit_bytes(&[0xA2, 0x00])
    // }
    //
    // fn stop_accepting_commands(&self) -> anyhow::Result<()> {
    //     self.transmit_bytes(&[0xA3, 0x00])
    // }
    //
    // pub fn shutdown(&self) -> anyhow::Result<()> {
    //     self.stop_accepting_commands()?;
    //     self.go_offline()
    // }
}
