use gabriele::hal::Hal;
use gabriele::printing::Instruction;
use std::io::Read;
use tokio::sync::mpsc;
use virtual_serialport::VirtualPort;

#[tokio::test]
async fn pushes_bytes_into_serial_port() {
    let (tx, rx) = mpsc::unbounded_channel::<Instruction>();

    let (port_use, mut port_listen) = VirtualPort::pair(9600, 64).unwrap();

    let mut hal = Hal::test_new(rx, port_use.into_boxed());

    let handle = tokio::task::spawn_blocking(move || hal.run());
    tx.send(Instruction::SendBytes([0x10, 0xa1])).unwrap();
    tx.send(Instruction::Shutdown).unwrap();
    _ = tokio::join!(handle);

    let mut read_buffer = [0u8; 6];
    port_listen.read_exact(&mut read_buffer).unwrap();

    assert_eq!(&read_buffer, &[0x10, 0xa1, 0xA3, 0x00, 0xA0, 0x00]);
}
