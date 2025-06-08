use gabriele::printing::{Instruction, SendBytesDetails};
use std::io::{Read, Write};

mod helpers;
use helpers::start_test_hal;

#[tokio::test]
async fn pushes_bytes_into_serial_port() {
    let (tx, handle, mut port) = start_test_hal();
    tx.send(Instruction::SendBytes(SendBytesDetails {
        idle_before: None,
        cmd: [0x10, 0xA1],
        idle_after: None,
    }))
    .unwrap();
    tx.send(Instruction::Halt).unwrap();
    _ = tokio::join!(handle);

    let mut read_buf: Vec<u8> = vec![];
    port.read_to_end(&mut read_buf).unwrap();

    assert_eq!(&read_buf, &[0x10, 0xA1]);
}

#[tokio::test]
async fn sends_correct_start_sequence_on_successful_feedback_response() {
    let (tx, handle, mut port) = start_test_hal();
    tx.send(Instruction::Prepare).unwrap();
    tx.send(Instruction::Halt).unwrap();
    // mock the feedback response
    port.write_all(&[161_u8]).unwrap();

    _ = tokio::join!(handle);

    let mut read_buf: Vec<u8> = vec![];
    port.read_to_end(&mut read_buf).unwrap();
    assert_eq!(&read_buf, &[0xA1, 0x00, 0xA4, 0x00, 0xA2, 0x00]);
}

#[tokio::test]
async fn breaks_start_sequence_on_absence_of_the_feedback_response() {
    let (tx, handle, mut port) = start_test_hal();
    tx.send(Instruction::Prepare).unwrap();
    tx.send(Instruction::Halt).unwrap();

    _ = tokio::join!(handle);

    let mut read_buf: Vec<u8> = vec![];
    port.read_to_end(&mut read_buf).unwrap();
    assert_eq!(&read_buf, &[0xA1, 0x00, 0xA4, 0x00]);
}

#[tokio::test]
async fn sends_correct_shutdown_sequence() {
    let (tx, handle, mut port) = start_test_hal();
    tx.send(Instruction::Shutdown).unwrap();
    // mock the feedback response
    port.write_all(&[161_u8]).unwrap();

    _ = tokio::join!(handle);

    let mut read_buf: Vec<u8> = vec![];
    port.read_to_end(&mut read_buf).unwrap();
    assert_eq!(&read_buf, &[0xA3, 0x00, 0xA0, 0x00]);
}
