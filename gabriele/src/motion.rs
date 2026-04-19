use crate::cmd::{Cmd, CmdJump, CmdMotion};
use crate::position::Position;
use crate::printing::Instruction;

fn move_carriage(increment: i16) -> impl Iterator<Item = Instruction> {
    [CmdMotion::delta_x(increment)]
        .into_iter()
        .flatten()
        .map(Cmd::Motion)
        .map(|cmd| cmd.as_instruction())
}

fn move_paper(increment: i16) -> impl Iterator<Item = Instruction> {
    [CmdMotion::delta_y(increment)]
        .into_iter()
        .flatten()
        .map(Cmd::Motion)
        .map(|cmd| cmd.as_instruction())
}

pub fn move_relative(x: i16, y: i16) -> impl Iterator<Item = Instruction> {
    move_carriage(x).chain(move_paper(y))
}

pub fn move_absolute(actual: &Position, target: &Position) -> impl Iterator<Item = Instruction> {
    let (x, y) = target.diff(actual);
    move_relative(x as i16, y as i16)
}

pub fn space_jump_left() -> impl Iterator<Item = Instruction> {
    [Cmd::Jump(CmdJump::Minus).as_instruction()].into_iter()
}

pub fn space_jump_right() -> impl Iterator<Item = Instruction> {
    [Cmd::Jump(CmdJump::Plus).as_instruction()].into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::printing::Instruction::*;
    use crate::resolution::{DEFAULT_X_RESOLUTION as X_RES, DEFAULT_Y_RESOLUTION as Y_RES};

    #[test]
    fn it_modes_the_carriage_one_space_rightwards() {
        let mut cmd = space_jump_right();

        let det = u16::from_be_bytes([0x83, 0]);
        assert_eq!(cmd.next().unwrap(), SendBytes(det));
        assert!(cmd.next().is_none());
    }

    #[test]
    fn it_modes_the_carriage_one_space_leftwards() {
        let mut cmd = space_jump_left();

        let det = u16::from_be_bytes([0x84, 0]);
        assert_eq!(cmd.next().unwrap(), SendBytes(det));
        assert!(cmd.next().is_none());
    }

    #[test]
    fn it_moves_in_relative_increments() {
        let mut cmd = move_relative(120, 32);

        let first = SendBytes(u16::from_be_bytes([0xc0, 120]));
        let second = SendBytes(u16::from_be_bytes([0xd0, 32]));
        assert_eq!(cmd.next().unwrap(), first);
        assert_eq!(cmd.next().unwrap(), second);
        assert!(cmd.next().is_none());
    }

    #[test]
    fn it_moves_the_carriage_one_character_place_rightwards() {
        let mut cmd = move_carriage(1 * X_RES as i16);

        let det = u16::from_be_bytes([0xc0, 12]);

        assert_eq!(cmd.next().unwrap(), SendBytes(det));
        assert!(cmd.next().is_none());
    }

    #[test]
    fn it_moves_the_carriage_one_character_place_leftwards() {
        let mut cmd = move_carriage(-1 * X_RES as i16);

        let det = u16::from_be_bytes([0xe0, 12]);

        assert_eq!(cmd.next().unwrap(), SendBytes(det));
        assert!(cmd.next().is_none());
    }

    #[test]
    fn it_rolls_the_paper_one_line_downwards() {
        let mut cmd = move_paper(1 * Y_RES as i16);

        let det = u16::from_be_bytes([0xd0, 16]);

        assert_eq!(cmd.next().unwrap(), SendBytes(det));
        assert!(cmd.next().is_none());
    }

    #[test]
    fn it_rolls_the_paper_one_line_upwards() {
        let mut cmd = move_paper(-1 * Y_RES as i16);

        let det = u16::from_be_bytes([0xf0, 16]);

        assert_eq!(cmd.next().unwrap(), SendBytes(det));
        assert!(cmd.next().is_none());
    }

    // #[test]
    // fn test_plus_y() {
    //     let data = Cmd::Motion(CmdMotion::plus_y(0x514));
    //     assert_eq!(data.to_bytes().unwrap(), [0xD5, 0x14]);
    // }
    //
    // #[test]
    // fn test_plus_y_overflow() {
    //     let data = Cmd::Motion(CmdMotion::plus_y(0x1514));
    //     let result = data.to_bytes();
    //     assert!(result.is_err());
    //     let msg = "bit size of input is larger than bit requested size";
    //     assert!(result.err().unwrap().to_string().contains(msg));
    // }
}
