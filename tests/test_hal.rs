use gabriele::printing::Instruction;
use serialport::{ClearBuffer, SerialPort};
use std::io::{Read, Write};

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

#[tokio::test]
async fn awaits_prepare_response_from_typewriter() {
    let (tx, handle, mut port) = start_test_hal();
    tx.send(Instruction::Prepare).unwrap();
    tx.send(Instruction::Halt).unwrap();

    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    let mut read_buffer = [0u8; 6];
    port.read_exact(&mut read_buffer).unwrap();

    assert_eq!(&read_buffer, &[0xA3, 0x00, 0xA0, 0x00, 0xA1, 0x00]);

    port.clear(ClearBuffer::All).unwrap();
    port.write_all(&[161_u8]).unwrap();

    _ = tokio::join!(handle);

    let mut read_buffer = [0u8; 2];
    port.read_exact(&mut read_buffer).unwrap();
    assert_eq!(&read_buffer, &[0xA4, 0x00]);
}
