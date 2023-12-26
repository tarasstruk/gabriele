use crate::position::Position;
use crate::printing::Instruction;

fn wrap_motion(value: u16) -> Box<dyn Iterator<Item = Instruction>> {
    Box::new(
        [
            Instruction::wait_short(),
            value.into(),
            Instruction::wait_long(),
        ]
        .into_iter(),
    )
}

fn roll_forward(steps: u16) -> Box<dyn Iterator<Item = Instruction>> {
    println!("roll the paper forward by {:?}", &steps);
    wrap_motion(steps | 0b1101_0000_0000_0000)
}

fn roll_backward(steps: u16) -> Box<dyn Iterator<Item = Instruction>> {
    println!("roll the paper backward by {:?}", &steps);
    wrap_motion(steps | 0b1111_0000_0000_0000)
}

fn carriage_forward(steps: u16) -> Box<dyn Iterator<Item = Instruction>> {
    println!("move the carriage forward by {:?}", &steps);
    wrap_motion(steps | 0b1100_0000_0000_0000)
}
fn carriage_backward(steps: u16) -> Box<dyn Iterator<Item = Instruction>> {
    println!("move the carriage <-backward by {:?}", &steps);
    wrap_motion(steps | 0b1110_0000_0000_0000)
}

pub fn move_carriage(increment: i32) -> Box<dyn Iterator<Item = Instruction>> {
    let value = u16::try_from(increment.abs()).unwrap();
    if increment < 0 {
        return carriage_backward(value);
    }
    if increment > 0 {
        return carriage_forward(value);
    }
    Box::new([Instruction::Empty].into_iter())
}

pub fn move_paper(increment: i32) -> Box<dyn Iterator<Item = Instruction>> {
    let value = u16::try_from(increment.abs()).unwrap();
    if increment < 0 {
        return roll_backward(value);
    }
    if increment > 0 {
        return roll_forward(value);
    }
    Box::new([Instruction::Empty].into_iter())
}

pub fn move_relative(x: i32, y: i32) -> Box<dyn Iterator<Item = Instruction>> {
    let items: Vec<Instruction> = move_carriage(x).chain(move_paper(y)).collect();
    Box::new(items.into_iter())
}

pub fn move_absolute(actual: Position, target: Position) -> Box<dyn Iterator<Item = Instruction>> {
    let (x, y) = target.diff(&actual);
    move_relative(x, y)
}

#[allow(unused)]
pub fn space_jump_left() -> Box<dyn Iterator<Item = Instruction>> {
    Box::new([Instruction::bytes(0b1000_0100, 0b0000_0000)].into_iter())
}

pub fn space_jump_right() -> Box<dyn Iterator<Item = Instruction>> {
    Box::new([Instruction::bytes(0b1000_0011, 0b0000_0000)].into_iter())
}
