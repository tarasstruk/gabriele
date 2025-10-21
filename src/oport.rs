use anyhow::Result;
use ractor::{Actor, ActorProcessingErr, ActorRef, RpcReplyPort};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_serial::SerialPort;

pub enum ActorMessage {
    WriteByte(u8, RpcReplyPort<bool>),
}

pub struct ActorState {
    port: Box<dyn SerialPort>,
}

pub struct UartActor;

impl Actor for UartActor {
    type State = ActorState;
    type Msg = ActorMessage;
    type Arguments = Box<dyn SerialPort>;

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
                if state.port.write(&[byte]).is_err() {
                    reply.send(false)?;
                    return Ok(());
                }
                println!("transmitted: {:?}", byte);

                let mut echo = [0_u8];
                if let Ok(n) = state.port.read(&mut echo) {
                    if n != 1 || echo[0] != byte {
                        println!("bytes do not match");
                        reply.send(false)?;
                    } else {
                        println!("received: {:?}", byte);
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
                println!("finish watching");
                break;
            }

            let mut buf = [0_u8];

            // echo the received byte from rx bac into tx
            if slave.read(&mut buf).await.is_ok() && slave.write(&buf).await.is_ok() {
                num_cycles -= 1;
                println!("Received {:?}", buf);
                let mut piece: Vec<u8> = buf.into();
                stash.append(&mut piece);
            }
        }
        stash
    }

    #[tokio::test]
    async fn writes_to_serial_port() {
        let (master, slave) = SerialStream::pair().unwrap();
        let handle = tokio::task::spawn(async move { spin(slave, 8).await });

        let master = Box::new(master);

        let (actor, actor_handle) = Actor::spawn(None, UartActor, master)
            .await
            .expect("Actor failed to start");

        for i in 0..8 {
            let outcome =
                ractor::call_t!(actor, ActorMessage::WriteByte, 100, i).expect("RPC failed");
            assert!(outcome);
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let x = handle.await.unwrap();
        assert_eq!(x, vec![0, 1, 2, 3, 4, 5, 6, 7]);

        actor.stop(None);
        actor_handle.await.unwrap();
    }
}
