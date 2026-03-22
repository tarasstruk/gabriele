use crate::cmd::{Cmd, CmdJump, CmdMotion};
use crate::position::Position;
use crate::printing::{Instruction, SendBytesDetails};
use deku::DekuContainerWrite;

#[derive(Debug, PartialEq)]
pub enum SequenceMotion {
    OneDim(Instruction),
    TwoDim(Instruction, Instruction),
    NoMotion,
}

impl From<SequenceMotion> for Vec<Instruction> {
    fn from(seq: SequenceMotion) -> Self {
        match seq {
            SequenceMotion::OneDim(inst) => vec![inst],
            SequenceMotion::TwoDim(i1, i2) => vec![i1, i2],
            SequenceMotion::NoMotion => vec![],
        }
    }
}

impl From<Option<Instruction>> for SequenceMotion {
    fn from(value: Option<Instruction>) -> Self {
        match value {
            None => SequenceMotion::NoMotion,
            Some(instr) => SequenceMotion::OneDim(instr),
        }
    }
}

impl From<[Option<Instruction>; 2]> for SequenceMotion {
    fn from(value: [Option<Instruction>; 2]) -> Self {
        match value {
            [Some(ins1), Some(ins2)] => SequenceMotion::TwoDim(ins1, ins2),
            [Some(ins1), None] => ins1.into(),
            [None, Some(ins2)] => ins2.into(),
            _ => SequenceMotion::NoMotion,
        }
    }
}

impl From<Instruction> for SequenceMotion {
    fn from(value: Instruction) -> Self {
        SequenceMotion::OneDim(value)
    }
}

fn wrap_decu(value: impl DekuContainerWrite) -> Instruction {
    let mut cmd = SendBytesDetails::default();
    value.to_slice(&mut cmd.cmd).unwrap();
    Instruction::SendBytes(cmd)
}

fn move_carriage(increment: i16) -> Option<Instruction> {
    let cmd = Cmd::Motion(CmdMotion::delta_x(increment)?);
    Some(wrap_decu(cmd))
}

fn move_paper(increment: i16) -> Option<Instruction> {
    let cmd = Cmd::Motion(CmdMotion::delta_y(increment)?);
    Some(wrap_decu(cmd))
}

pub fn move_relative(x: i16, y: i16) -> SequenceMotion {
    [move_carriage(x), move_paper(y)].into()
}

pub fn move_absolute(actual: &Position, target: &Position) -> SequenceMotion {
    let (x, y) = target.diff(actual);
    move_relative(x as i16, y as i16)
}

pub fn space_jump_left() -> SequenceMotion {
    wrap_decu(Cmd::Jump(CmdJump::Minus)).into()
}

pub fn space_jump_right() -> SequenceMotion {
    wrap_decu(Cmd::Jump(CmdJump::Plus)).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::printing::Instruction::*;
    use crate::resolution::{DEFAULT_X_RESOLUTION as X_RES, DEFAULT_Y_RESOLUTION as Y_RES};
    use deku::DekuContainerWrite;
    use SequenceMotion::*;

    #[test]
    fn it_modes_the_carriage_one_space_rightwards() {
        let cmd = space_jump_right();

        let det = SendBytesDetails { cmd: [0x83, 0] };
        assert_eq!(cmd, OneDim(SendBytes(det)));
    }

    #[test]
    fn it_modes_the_carriage_one_space_leftwards() {
        let cmd = space_jump_left();

        let det = SendBytesDetails { cmd: [0x84, 0] };
        assert_eq!(cmd, OneDim(SendBytes(det)));
    }

    #[test]
    fn it_moves_in_relative_increments() {
        let cmd = move_relative(120, 32);

        let first = SendBytes(SendBytesDetails { cmd: [0xc0, 120] });
        let second = SendBytes(SendBytesDetails { cmd: [0xd0, 32] });
        assert_eq!(cmd, TwoDim(first, second));
    }

    #[test]
    fn it_moves_the_carriage_one_character_place_rightwards() {
        let cmd = move_carriage(1 * X_RES as i16);

        let det = SendBytesDetails { cmd: [0xc0, 12] };

        assert_eq!(cmd, Some(SendBytes(det)));
    }

    #[test]
    fn it_moves_the_carriage_one_character_place_leftwards() {
        let cmd = move_carriage(-1 * X_RES as i16);

        let det = SendBytesDetails { cmd: [0xe0, 12] };

        assert_eq!(cmd, Some(SendBytes(det)));
    }

    #[test]
    fn it_rolls_the_paper_one_line_downwards() {
        let cmd = move_paper(1 * Y_RES as i16);

        let det = SendBytesDetails { cmd: [0xd0, 16] };

        assert_eq!(cmd, Some(SendBytes(det)));
    }

    #[test]
    fn it_rolls_the_paper_one_line_upwards() {
        let cmd = move_paper(-1 * Y_RES as i16);

        let det = SendBytesDetails { cmd: [0xf0, 16] };

        assert_eq!(cmd, Some(SendBytes(det)));
    }

    #[test]
    fn test_plus_y() {
        let data = Cmd::Motion(CmdMotion::plus_y(0x514));
        assert_eq!(data.to_bytes().unwrap(), [0xD5, 0x14]);
    }

    #[test]
    fn test_plus_y_overflow() {
        let data = Cmd::Motion(CmdMotion::plus_y(0x1514));
        let result = data.to_bytes();
        assert!(result.is_err());
        let msg = "bit size of input is larger than bit requested size";
        assert!(result.err().unwrap().to_string().contains(msg));
    }
}
