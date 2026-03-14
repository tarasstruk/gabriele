mod helpers;

use crate::helpers::app::TestApp;
use bytes::{Bytes, BytesMut};
use gabriele::impression::Impression;
use gabriele::motion::{move_carriage, move_paper};
use gabriele::printing::{Instruction, SendBytesDetails};
use gabriele::symbol::AfterSymbolPrinted;
use gabriele::{
    position::Position,
    resolution::{DEFAULT_X_RESOLUTION as X_RES, DEFAULT_Y_RESOLUTION as Y_RES},
};
use helpers::load_test_db;

#[tokio::test]
async fn prints_two_characters() {
    let mut app = TestApp::run(1234).await;
    let db = load_test_db();

    app.machine.print("AT", &db);

    let hit = Impression::default().value() | AfterSymbolPrinted::default().value();

    let byte = app.rx.recv().await.unwrap();
    assert_eq!(byte, 36);

    let byte = app.rx.recv().await.unwrap();
    assert_eq!(byte, hit);

    let byte = app.rx.recv().await.unwrap();
    assert_eq!(byte, 37);

    let byte = app.rx.recv().await.unwrap();
    assert_eq!(byte, hit);

    let expected_position = Position {
        x: X_RES * 2,
        y: 0,
        ..Default::default()
    };
    assert_eq!(app.machine.current_position(), expected_position);

    app.teardown().await;
}

#[tokio::test]
async fn prints_special_character() {
    let mut app = TestApp::run(1235).await;
    let db = load_test_db();

    app.machine.print("à", &db);

    let first_hit = Impression::default().value() | AfterSymbolPrinted::HoldOn.value();
    let second_hit = Impression::Mild.value() | AfterSymbolPrinted::MoveRight.value();

    let byte = app.rx.recv().await.unwrap();
    assert_eq!(byte, 94);

    let byte = app.rx.recv().await.unwrap();
    assert_eq!(byte, first_hit);

    let byte = app.rx.recv().await.unwrap();
    assert_eq!(byte, 72);

    let byte = app.rx.recv().await.unwrap();
    assert_eq!(byte, second_hit);

    let expected_position = Position {
        x: X_RES * 1,
        y: 0,
        ..Default::default()
    };
    assert_eq!(app.machine.current_position(), expected_position);

    app.teardown().await;
}

#[tokio::test]
async fn prints_character_with_a_newline() {
    let mut app = TestApp::run(1236).await;
    let db = load_test_db();

    app.machine.print("A\n", &db);
    app.machine.shutdown();

    // 1 printing instruction plus 2 motion instructions
    let hit = Impression::default().value() | AfterSymbolPrinted::MoveRight.value();
    let carriage_motion: Vec<Instruction> = move_carriage(-1 * X_RES).collect();
    let roll_motion: Vec<Instruction> = move_paper(1 * Y_RES).collect();

    let byte = app.rx.recv().await.unwrap();
    assert_eq!(byte, 36);

    let byte = app.rx.recv().await.unwrap();
    assert_eq!(byte, hit);

    let mut iter = carriage_motion.into_iter().chain(roll_motion.into_iter());

    let mut counter = 0;
    while let Some(Instruction::SendBytes(SendBytesDetails { cmd })) = iter.next() {
        let byte = app.rx.recv().await.unwrap();
        assert_eq!(byte, cmd[0]);

        let byte = app.rx.recv().await.unwrap();
        assert_eq!(byte, cmd[1]);

        counter += 1;
    }

    // should be received 2 x 2 bytes
    assert_eq!(counter, 2);

    app.teardown().await;
}

#[tokio::test]
async fn prints_welcome_file() {
    let _ = env_logger::builder().is_test(true).try_init();
    log::info!("Це повідомлення з логу!");

    let mut app = TestApp::run(1237).await;
    let db = load_test_db();

    let content = include_str!("../welcome.txt");
    let expected = Bytes::from_static(include_bytes!("../ref_output.bin"));

    app.machine.offset(4 * 12);
    app.machine.print(content, &db);
    app.halt();

    let mut buf = BytesMut::with_capacity(1024);

    while let Some(byte) = app.rx.recv().await {
        buf.extend_from_slice(&[byte]);
    }

    assert_eq!(buf, expected);
    app.teardown().await;
}
