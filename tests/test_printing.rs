mod helpers;
use crate::helpers::{load_test_db, start_test_app};
use gabriele::{
    daisy::{AfterSymbolPrinted, Impression},
    printing::Instruction,
};

#[tokio::test]
async fn prints_two_characters() {
    let (mut app, runner) = start_test_app();
    let db = load_test_db();

    app.machine.print("AT", &db);
    app.machine.shutdown();

    _ = tokio::join!(runner);
    let latch = app.latch.lock().unwrap();

    assert_eq!(latch.len(), 2);
    let second_byte = Impression::default().value() | AfterSymbolPrinted::default().value();
    assert_eq!(latch.get(0).unwrap(), &Instruction::bytes(36, second_byte));
    assert_eq!(latch.get(1).unwrap(), &Instruction::bytes(37, second_byte));
}
