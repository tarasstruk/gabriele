use anyhow::Result;
use log::debug;
use ractor::{Actor, ActorProcessingErr, ActorRef, RpcReplyPort};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_serial::SerialStream;

pub enum ActorMessage {
    WriteByte(u8, RpcReplyPort<bool>),
}

pub struct ActorState {
    port: SerialStream,
}

pub struct UartActor;

impl Actor for UartActor {
    type State = ActorState;
    type Msg = ActorMessage;
    type Arguments = SerialStream;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        arguments: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(ActorState { port: arguments })
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            ActorMessage::WriteByte(byte, reply) => {
                if state.port.write(&[byte]).await.is_err() {
                    reply.send(false)?;
                    return Ok(());
                }
                debug!("transmitted: {:?}", byte);

                let mut echo = [0_u8];
                if let Ok(n) = state.port.read(&mut echo).await {
                    if n != 1 || echo[0] != byte {
                        debug!("bytes do not match");
                        reply.send(false)?;
                    } else {
                        debug!("received: {:?}", byte);
                        reply.send(true)?;
                    }
                } else {
                    reply.send(false)?;
                }
            }
        }
        Ok(())
    }
}

mod test {
    use super::*;
    use tokio_serial::SerialStream;

    #[allow(dead_code)]
    async fn spin(mut slave: SerialStream, mut num_cycles: usize) -> Vec<u8> {
        let mut stash: Vec<u8> = Vec::with_capacity(num_cycles);
        loop {
            if num_cycles == 0 {
                debug!("finish watching");
                break;
            }

            let mut buf = [0_u8];

            // echo the received byte from rx bac into tx
            if slave.read(&mut buf).await.is_ok() && slave.write(&buf).await.is_ok() {
                num_cycles -= 1;
                debug!("Received {:?}", buf);
                let mut piece: Vec<u8> = buf.into();
                stash.append(&mut piece);
            }
        }
        stash
    }
}
