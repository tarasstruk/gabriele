use crate::position::Position;
use crate::printing::{Instruction, SendBytesDetails};
use crate::times;
use log::debug;

fn wrap_motion(value: u16) -> Box<dyn Iterator<Item = Instruction>> {
    let mut cmd: SendBytesDetails = value.into();
    cmd.idle_before = Some(times::SHORT_MS);
    cmd.idle_after = Some(times::LONG_MS);

    Box::new([Instruction::SendBytes(cmd)].into_iter())
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

        let det = SendBytesDetails {
            idle_before: None,
            cmd: [0x83, 0],
            idle_after: None,
        };
        assert_eq!(cmd.next(), Some(SendBytes(det)));
        assert_eq!(cmd.next(), None);
    }

    #[test]
    fn it_modes_the_carriage_one_space_leftwards() {
        let mut cmd = space_jump_left();

        let det = SendBytesDetails {
            idle_before: None,
            cmd: [0x84, 0],
            idle_after: None,
        };
        assert_eq!(cmd.next(), Some(SendBytes(det)));
        assert_eq!(cmd.next(), None);
    }

    #[test]
    fn it_moves_in_relative_increments() {
        let mut cmd = move_relative(120, 32);
        let mut det = SendBytesDetails {
            idle_before: Some(SHORT_MS),
            cmd: [0xc0, 120],
            idle_after: Some(LONG_MS),
        };
        assert_eq!(cmd.next(), Some(SendBytes(det)));

        det.cmd = [0xd0, 32];
        assert_eq!(cmd.next(), Some(SendBytes(det)));
        assert_eq!(cmd.next(), None);
    }

    #[test]
    fn it_moves_the_carriage_one_character_place_rightwards() {
        let mut cmd = move_carriage(1 * X_RES);

        let det = SendBytesDetails {
            idle_before: Some(SHORT_MS),
            cmd: [0xc0, 12],
            idle_after: Some(LONG_MS),
        };

        assert_eq!(cmd.next(), Some(SendBytes(det)));
        assert_eq!(cmd.next(), None);
    }

    #[test]
    fn it_moves_the_carriage_one_character_place_leftwards() {
        let mut cmd = move_carriage(-1 * X_RES);

        let det = SendBytesDetails {
            idle_before: Some(SHORT_MS),
            cmd: [0xe0, 12],
            idle_after: Some(LONG_MS),
        };

        assert_eq!(cmd.next(), Some(SendBytes(det)));
        assert_eq!(cmd.next(), None);
    }

    #[test]
    fn it_rolls_the_paper_one_line_downwards() {
        let mut cmd = move_paper(1 * Y_RES);

        let det = SendBytesDetails {
            idle_before: Some(SHORT_MS),
            cmd: [0xd0, 16],
            idle_after: Some(LONG_MS),
        };

        assert_eq!(cmd.next(), Some(SendBytes(det)));
        assert_eq!(cmd.next(), None);
    }

    #[test]
    fn it_rolls_the_paper_one_line_upwards() {
        let mut cmd = move_paper(-1 * Y_RES);

        let det = SendBytesDetails {
            idle_before: Some(SHORT_MS),
            cmd: [0xf0, 16],
            idle_after: Some(LONG_MS),
        };

        assert_eq!(cmd.next(), Some(SendBytes(det)));
        assert_eq!(cmd.next(), None);
    }
}
