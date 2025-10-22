use gabriele::printing::{Instruction, SendBytesDetails};
mod helpers;
use helpers::start_test_hal;

#[tokio::test]
async fn pushes_bytes_into_serial_port() {
    let (tx, handle) = start_test_hal(6);
    tx.send(Instruction::SendBytes(SendBytesDetails {
        idle_before: None,
        cmd: [0x10, 0xA1],
        idle_after: None,
    }))
    .unwrap();
    tx.send(Instruction::Halt).unwrap();

    let read_buf = handle.await.unwrap();
    assert_eq!(&read_buf, &[0xA1, 0, 0xA2, 0, 0x10, 0xA1]);
}

#[tokio::test]
async fn sends_correct_start_sequence() {
    let (tx, handle) = start_test_hal(4);
    tx.send(Instruction::Prepare).unwrap();
    tx.send(Instruction::Halt).unwrap();

    let read_buf = handle.await.unwrap();
    assert_eq!(&read_buf, &[0xA1, 0x00, 0xA2, 0x00]);
}

#[tokio::test]
async fn sends_correct_shutdown_sequence() {
    let (tx, handle) = start_test_hal(8);
    tx.send(Instruction::Shutdown).unwrap();
    // mock the feedback response

    let read_buf = handle.await.unwrap();
    assert_eq!(&read_buf, &[0xA1, 0, 0xA2, 0, 0xA3, 0x00, 0xA0, 0x00]);
}
