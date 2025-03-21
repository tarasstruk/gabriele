use crate::position::Position;
use crate::printing::Instruction;
use log::debug;

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
    debug!("roll the paper forward by {:?}", &steps);
    wrap_motion(steps | 0b1101_0000_0000_0000)
}

fn roll_backward(steps: u16) -> Box<dyn Iterator<Item = Instruction>> {
    debug!("roll the paper backward by {:?}", &steps);
    wrap_motion(steps | 0b1111_0000_0000_0000)
}

fn carriage_forward(steps: u16) -> Box<dyn Iterator<Item = Instruction>> {
    debug!("move the carriage forward by {:?}", &steps);
    wrap_motion(steps | 0b1100_0000_0000_0000)
}
fn carriage_backward(steps: u16) -> Box<dyn Iterator<Item = Instruction>> {
    debug!("move the carriage <-backward by {:?}", &steps);
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

pub fn move_absolute(
    actual: &Position,
    target: &Position,
) -> Box<dyn Iterator<Item = Instruction>> {
    let (x, y) = target.diff(actual);
    move_relative(x, y)
}

#[allow(unused)]
pub fn space_jump_left() -> Box<dyn Iterator<Item = Instruction>> {
    Box::new([Instruction::bytes(0b1000_0100, 0b0000_0000)].into_iter())
}

pub fn space_jump_right() -> Box<dyn Iterator<Item = Instruction>> {
    Box::new([Instruction::bytes(0b1000_0011, 0b0000_0000)].into_iter())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::printing::Instruction::*;
    use crate::resolution::{DEFAULT_X_RESOLUTION as X_RES, DEFAULT_Y_RESOLUTION as Y_RES};
    use crate::times::*;

    #[test]
    fn it_modes_the_carriage_one_space_rightwards() {
        let mut cmd = space_jump_right();
        assert_eq!(cmd.next(), Some(SendBytes([0x83, 0])));
        assert_eq!(cmd.next(), None);
    }

    #[test]
    fn it_modes_the_carriage_one_space_leftwards() {
        let mut cmd = space_jump_left();
        assert_eq!(cmd.next(), Some(SendBytes([0x84, 0])));
        assert_eq!(cmd.next(), None);
    }

    #[test]
    fn it_moves_in_relative_increments() {
        let mut cmd = move_relative(120, 32);
        assert_eq!(cmd.next(), Some(Idle(SHORT_MS)));
        assert_eq!(cmd.next(), Some(SendBytes([0xc0, 120])));
        assert_eq!(cmd.next(), Some(Idle(LONG_MS)));

        assert_eq!(cmd.next(), Some(Idle(SHORT_MS)));
        assert_eq!(cmd.next(), Some(SendBytes([0xd0, 32])));
        assert_eq!(cmd.next(), Some(Idle(LONG_MS)));
        assert_eq!(cmd.next(), None);
    }

    #[test]
    fn it_moves_the_carriage_one_character_place_rightwards() {
        let mut cmd = move_carriage(1 * X_RES);
        assert_eq!(cmd.next(), Some(Idle(SHORT_MS)));
        assert_eq!(cmd.next(), Some(SendBytes([0xc0, 12])));
        assert_eq!(cmd.next(), Some(Idle(LONG_MS)));
        assert_eq!(cmd.next(), None);
    }

    #[test]
    fn it_moves_the_carriage_one_character_place_leftwards() {
        let mut cmd = move_carriage(-1 * X_RES);
        assert_eq!(cmd.next(), Some(Idle(SHORT_MS)));
        assert_eq!(cmd.next(), Some(SendBytes([0xe0, 12])));
        assert_eq!(cmd.next(), Some(Idle(LONG_MS)));
        assert_eq!(cmd.next(), None);
    }

    #[test]
    fn it_rolls_the_paper_one_line_downwards() {
        let mut cmd = move_paper(1 * Y_RES);
        assert_eq!(cmd.next(), Some(Idle(SHORT_MS)));
        assert_eq!(cmd.next(), Some(SendBytes([0xd0, 16])));
        assert_eq!(cmd.next(), Some(Idle(LONG_MS)));
        assert_eq!(cmd.next(), None);
    }

    #[test]
    fn it_rolls_the_paper_one_line_upwards() {
        let mut cmd = move_paper(-1 * Y_RES);
        assert_eq!(cmd.next(), Some(Idle(SHORT_MS)));
        assert_eq!(cmd.next(), Some(SendBytes([0xf0, 16])));
        assert_eq!(cmd.next(), Some(Idle(LONG_MS)));
        assert_eq!(cmd.next(), None);
    }
}
