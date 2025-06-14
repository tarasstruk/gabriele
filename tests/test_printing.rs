mod helpers;
use gabriele::impression::Impression;
use gabriele::symbol::AfterSymbolPrinted;
use gabriele::{
    motion::{move_carriage, move_paper},
    position::Position,
    printing::Instruction,
    resolution::{DEFAULT_X_RESOLUTION as X_RES, DEFAULT_Y_RESOLUTION as Y_RES},
};
use helpers::{load_test_db, start_test_app};

#[tokio::test]
async fn prints_two_characters() {
    let (mut app, runner) = start_test_app();
    let db = load_test_db();

    app.machine.print("AT", &db);
    app.machine.shutdown();

    _ = tokio::join!(runner);
    let latch = app.latch.lock().unwrap();

    assert_eq!(latch.len(), 2);
    let hit = Impression::default().value() | AfterSymbolPrinted::default().value();
    assert_eq!(latch.get(0).unwrap(), &Instruction::bytes(36, hit));
    assert_eq!(latch.get(1).unwrap(), &Instruction::bytes(37, hit));

    let expected_position = Position {
        x: X_RES * 2,
        y: 0,
        ..Default::default()
    };
    assert_eq!(app.machine.current_position(), expected_position);
}

#[tokio::test]
async fn prints_special_character() {
    let (mut app, runner) = start_test_app();
    let db = load_test_db();

    app.machine.print("à", &db);
    app.machine.shutdown();

    _ = tokio::join!(runner);
    let latch = app.latch.lock().unwrap();

    assert_eq!(latch.len(), 2);
    let first_hit = Impression::default().value() | AfterSymbolPrinted::HoldOn.value();
    let second_hit = Impression::Mild.value() | AfterSymbolPrinted::MoveRight.value();
    assert_eq!(latch.get(0).unwrap(), &Instruction::bytes(94, first_hit));
    assert_eq!(latch.get(1).unwrap(), &Instruction::bytes(72, second_hit));

    let expected_position = Position {
        x: X_RES * 1,
        y: 0,
        ..Default::default()
    };
    assert_eq!(app.machine.current_position(), expected_position);
}

#[tokio::test]
async fn prints_character_with_a_newline() {
    let (mut app, runner) = start_test_app();
    let db = load_test_db();

    app.machine.print("A\n", &db);
    app.machine.shutdown();

    _ = tokio::join!(runner);
    let latch = app.latch.lock().unwrap();

    // 1 printing instruction plus 2 motion instructions
    assert_eq!(latch.len(), 3);
    let hit = Impression::default().value() | AfterSymbolPrinted::MoveRight.value();
    let carriage_motion: Vec<Instruction> = move_carriage(-1 * X_RES).collect();
    let roll_motion: Vec<Instruction> = move_paper(1 * Y_RES).collect();

    assert_eq!(latch.get(0).unwrap(), &Instruction::bytes(36, hit));
    assert_eq!(latch.get(1).unwrap(), carriage_motion.get(0).unwrap());
    assert_eq!(latch.get(2).unwrap(), roll_motion.get(0).unwrap());
}
