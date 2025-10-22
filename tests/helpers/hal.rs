use gabriele::hal::Hal;
use gabriele::printing::Instruction;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;
use tokio_serial::SerialStream;

async fn spin(mut slave: SerialStream, num_cycles: usize) -> Vec<u8> {
    let mut stash: Vec<u8> = Vec::with_capacity(num_cycles);
    for _i in 0..num_cycles {
        let mut buf = [0_u8];
        // echo the received byte from rx bac into tx
        if slave.read(&mut buf).await.is_ok() && slave.write(&buf).await.is_ok() {
            stash.push(buf[0]);
        }
    }
    stash
}

pub fn start_test_hal(num_cycles: usize) -> (UnboundedSender<Instruction>, JoinHandle<Vec<u8>>) {
    let (tx, rx) = mpsc::unbounded_channel::<Instruction>();

    let (master, slave) = SerialStream::pair().unwrap();

    let handle = tokio::task::spawn(async move { spin(slave, num_cycles).await });

    let mut hal = Hal::new(rx);
    tokio::task::spawn(async move { hal.run(master).await });

    (tx, handle)
}
