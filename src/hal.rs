use crate::oport::{ActorMessage, UartActor};
use crate::printing::Instruction;
use log::{debug, info};
use ractor::concurrency::tokio_primitives::JoinHandle;
use ractor::{Actor, ActorRef};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio_serial::SerialStream;

pub struct Hal {
    receiver: UnboundedReceiver<Instruction>,
    actor: Option<ActorRef<ActorMessage>>,
    actor_handle: Option<JoinHandle<()>>,
}

impl Hal {
    pub fn new(receiver: UnboundedReceiver<Instruction>) -> Self {
        Hal {
            receiver,
            actor: None,
            actor_handle: None,
        }
    }

    pub async fn run(&mut self, stream: SerialStream) {
        let (actor, actor_handle) = Actor::spawn(None, UartActor, stream)
            .await
            .expect("Actor failed to start");

        self.actor = Some(actor);
        self.actor_handle = Some(actor_handle);

        self.prepare().await;

        debug!("runner started");

        while let Some(item) = self.receiver.recv().await {
            debug!("received message: {:?}", &item);
            match item {
                Instruction::Halt => break,
                // Instruction::Prepare => self.prepare().await,
                Instruction::SendBytes(details) => self.transmit_bytes(&details.cmd).await,
                // Instruction::Idle(millis) => wait(millis),
                // Instruction::Empty => continue,
                Instruction::Shutdown => {
                    self.shutdown().await;
                    break;
                }
                _ => continue,
            }
        }
        self.graceful_shutdown_actor().await;
    }

    async fn graceful_shutdown_actor(&mut self) {
        if let Some(actor) = self.actor.take() {
            actor.stop(Some(String::from("graceful shutdown")));
        }

        if let Some(handle) = self.actor_handle.take() {
            handle.await.unwrap();
        }

        println!("Gabriele says good bye")
    }

    pub async fn transmit_bytes(&mut self, bytes: &[u8]) {
        if let Some(actor) = self.actor.take() {
            for byte in bytes {
                ractor::call_t!(actor, ActorMessage::WriteByte, 5000, *byte)
                    .expect("TX timeout exceeded");
            }
            self.actor = Some(actor)
        }
    }

    pub async fn prepare(&mut self) {
        self.go_online().await;
        self.start_accepting_commands().await;
    }

    async fn go_offline(&mut self) {
        info!("go off-line");
        self.transmit_bytes(&[0xA0, 0x00]).await;
    }

    async fn go_online(&mut self) {
        info!("go on-line");
        self.transmit_bytes(&[0xA1, 0x00]).await;
    }

    async fn start_accepting_commands(&mut self) {
        info!("start accepting printing commands");
        self.transmit_bytes(&[0xA2, 0x00]).await;
        info!("machine is now accepting the commands");
    }

    async fn stop_accepting_commands(&mut self) {
        info!("stop accepting printing commands");
        self.transmit_bytes(&[0xA3, 0x00]).await;
    }

    pub async fn shutdown(&mut self) {
        self.stop_accepting_commands().await;
        self.go_offline().await;
    }
}
