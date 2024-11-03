use gabriele::printing::Instruction;
use std::io::Read;
mod helpers;
use helpers::start_test_hal;

#[tokio::test]
async fn pushes_bytes_into_serial_port() {
    let (tx, handle, mut port) = start_test_hal();
    tx.send(Instruction::SendBytes([0x10, 0xa1])).unwrap();
    tx.send(Instruction::Shutdown).unwrap();
    _ = tokio::join!(handle);

    let mut read_buffer = [0u8; 6];
    port.read_exact(&mut read_buffer).unwrap();

    assert_eq!(&read_buffer, &[0x10, 0xa1, 0xA3, 0x00, 0xA0, 0x00]);
}
