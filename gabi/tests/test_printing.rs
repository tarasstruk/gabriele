mod helpers;

use crate::helpers::app::TestApp;
use bytes::{Bytes, BytesMut};
use gabriele::cmd::{Cmd, Impression};
use gabriele::motion::move_relative;
use gabriele::printing::Instruction;
use gabriele::symbol::{AfterSymbolPrinted, CmdSymbol, SymbolPrintingAttrs};
use gabriele::{
    position::Position,
    resolution::{DEFAULT_X_RESOLUTION as X_RES, DEFAULT_Y_RESOLUTION as Y_RES},
};

fn hit(impression: Impression, direction: AfterSymbolPrinted) -> u8 {
    let mut sym = CmdSymbol::default();
    sym.attr = SymbolPrintingAttrs {
        direction,
        impression,
    };
    Cmd::SymbolLow(sym).as_u16().to_be_bytes()[1]
}

#[tokio::test]
async fn prints_two_characters() {
    let mut app = TestApp::run(1234).await;

    app.machine.print("AT").await;

    let hit = hit(Default::default(), Default::default());

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
    app.machine.print("à").await;

    let first_hit = hit(Default::default(), AfterSymbolPrinted::HoldOn);

    let second_hit = hit(Impression::Mild, AfterSymbolPrinted::MoveRight);

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

    app.machine.print("A\n").await;
    app.machine.shutdown().await;

    let hit = crate::hit(Default::default(), AfterSymbolPrinted::MoveRight);

    let carriage_motion: Vec<Instruction> = move_relative(-1 * X_RES as i16, 0).collect();
    let roll_motion: Vec<Instruction> = move_relative(0, 1 * Y_RES as i16).collect();

    let byte = app.rx.recv().await.unwrap();
    assert_eq!(byte, 36);

    let byte = app.rx.recv().await.unwrap();
    assert_eq!(byte, hit);

    let mut iter = carriage_motion.into_iter().chain(roll_motion.into_iter());

    let mut counter = 0;
    while let Some(Instruction::SendBytes(word)) = iter.next() {
        let cmd = word.to_be_bytes();

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

    let mut app = TestApp::run(1237).await;

    let content = include_str!("../welcome.txt");
    let expected = Bytes::from_static(include_bytes!("../ref_output.bin"));

    app.machine.offset(4 * 12).await;
    app.machine.print(content).await;
    app.halt().await;

    let mut buf = BytesMut::with_capacity(1024);

    while let Some(byte) = app.rx.recv().await {
        buf.extend_from_slice(&[byte]);
    }

    assert_eq!(buf, expected);
    app.teardown().await;
}
