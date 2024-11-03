mod helpers;
use crate::helpers::{load_test_db, start_test_app};
use gabriele::{
    daisy::{AfterSymbolPrinted, Impression},
    motion::{move_carriage, move_paper},
    position::{Position, DEFAULT_X_RESOLUTION, DEFAULT_Y_RESOLUTION},
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
    let hit = Impression::default().value() | AfterSymbolPrinted::default().value();
    assert_eq!(latch.get(0).unwrap(), &Instruction::bytes(36, hit));
    assert_eq!(latch.get(1).unwrap(), &Instruction::bytes(37, hit));

    let expected_position = Position {
        x: DEFAULT_X_RESOLUTION * 2,
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
        x: DEFAULT_X_RESOLUTION * 1,
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

    assert_eq!(latch.len(), 7);
    let hit = Impression::default().value() | AfterSymbolPrinted::MoveRight.value();
    let carriage_motion: Vec<Instruction> = move_carriage(-1 * DEFAULT_X_RESOLUTION).collect();
    let roll_motion: Vec<Instruction> = move_paper(1 * DEFAULT_Y_RESOLUTION).collect();

    assert_eq!(latch.get(0).unwrap(), &Instruction::bytes(36, hit));

    // move the carriage to the `x = 0` position
    for (idx, inst) in (latch[1..=3]).iter().enumerate() {
        assert_eq!(inst, carriage_motion.get(idx).unwrap());
    }

    // roll the paper to `y = 1` position
    // `[Idle(200), SendBytes([208, 16]), Idle(1000)]`
    for (idx, inst) in (latch[4..=6]).iter().enumerate() {
        assert_eq!(inst, roll_motion.get(idx).unwrap());
    }
}
