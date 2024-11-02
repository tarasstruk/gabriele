mod helpers;
use crate::helpers::{load_test_db, start_test_app};
use gabriele::printing::Instruction;

#[tokio::test]
async fn starts_test_application() {
    let (mut app, runner) = start_test_app();
    let _db = load_test_db();

    app.machine.send_empty_instruction();
    app.machine.shutdown();

    _ = tokio::join!(runner);
    let latch = app.latch.lock().unwrap();

    assert_eq!(latch.len(), 1);
    assert_eq!(latch.get(0).unwrap(), &Instruction::Empty);
}
