#![allow(unused)]
#![allow(dead_code)]

use crate::daisy::{ActionMapping, AfterSymbolPrinted, Symbol};
use crate::machine::{PrintingDirection, Settings};
use crate::motion;
use crate::position::Position;
use crate::times::*;
use log::debug;

/// The basic directive for the machine
/// Idle specifies milliseconds
/// SendBytes specifies a sequence of 2 bytes to be send over a serial port
#[derive(PartialEq, Debug)]
pub enum Instruction {
    #[allow(unused)]
    Prepare,
    Idle(u64),
    SendBytes([u8; 2]),
    Empty,
    Shutdown,
    Halt,
}

impl Instruction {
    pub fn bytes(b1: u8, b2: u8) -> Self {
        Self::SendBytes([b1, b2])
    }
    pub fn wait_short() -> Self {
        Self::Idle(SHORT_MS)
    }

    pub fn wait_tiny() -> Self {
        Self::Idle(TINY_MS)
    }

    pub fn wait_long() -> Self {
        Self::Idle(LONG_MS)
    }
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        Self::SendBytes(value.to_be_bytes())
    }
}

// Action defines what we do with a Symbol
// each instance of Action can produce a series of Instructions.
//
// If a concrete Action cannot be performed in the concrete conditions
// then the execution of program should be stopped.
//
// In the future releases the Action should be able to respond with a recovery option.
// For example a print-symbol action: when the carriage has reached the left limit,
// we cannot perform this action. The recovery-action would the carriage return.
// In this case a recovery strategy would look like this:
// - the current action (printing a symbol) cannot be performed;
// - tt responds with empty set (iterator) for instructions method;
// - the executor party checks for a recovery actions (should be proposed by the current one)
// - the recovery sequence would contain a copy of the current action at the end of the list

/// Action represents a concrete primitive action to be performed by the Machine
/// in the current conditions, taking into account the base_position and the current_position.
pub struct Action {
    pub symbol: Symbol,
    pub settings: Settings,
}

impl Action {
    pub fn new(symbol: Symbol, settings: Settings) -> Self {
        Self { symbol, settings }
    }

    /// Generates a sequence of the Instructions,
    /// taking the current Position as a reference point.
    /// The result of these instructions is the printed Symbol or/and the associated motion.
    pub fn instructions(
        &self,
        base_position: &Position,
        current_position: &mut Position,
    ) -> impl Iterator<Item = Instruction> {
        let old_position = current_position.clone();
        self.update_position(base_position, current_position);
        debug!("action {:?}", self.symbol.act);
        match self.symbol.act {
            ActionMapping::Print => self.symbol.instructions(self.settings.direction),
            ActionMapping::Whitespace => match self.settings.direction {
                PrintingDirection::Right => motion::space_jump_right(),
                PrintingDirection::Left => motion::space_jump_left(),
            },
            ActionMapping::CarriageReturn => {
                motion::move_absolute(&old_position, &current_position)
            }
        }
    }

    /// New position represents a calculated desired Position
    /// where the machine is expected to be
    /// after the generated Instructions have been executed.
    pub fn update_position(&self, base_position: &Position, position: &mut Position) {
        let pos = match self.symbol.act {
            ActionMapping::Print => match self.settings.direction {
                PrintingDirection::Right => {
                    &position.increment_x(self.symbol.x_positions_increment())
                }
                PrintingDirection::Left => {
                    &position.decrement_x(self.symbol.x_positions_increment())
                }
            },

            ActionMapping::Whitespace => match self.settings.direction {
                PrintingDirection::Right => &position.step_right(),
                PrintingDirection::Left => &position.step_left(),
            },
            ActionMapping::CarriageReturn => &position.cr(base_position),
        };
        position.jump(&pos)
    }
}

#[cfg(test)]
use crate::times::*;
mod tests {
    use super::Action;
    use crate::daisy::Symbol;
    use crate::position::Position;
    use crate::printing::Instruction;
    use crate::times::{LONG_MS, SHORT_MS};

    #[test]
    fn test_print_symbol() {
        let symbol = Symbol::new('ü').petal(81);
        let mut pos: Position = Default::default();
        let base_pos: Position = Default::default();

        let action = Action::new(symbol, Default::default());
        let mut commands = action.instructions(&base_pos, &mut pos);
        let pos_diff = pos.diff(&base_pos);

        assert_eq!(pos_diff, (12, 0));
        assert_eq!(commands.next(), Some(Instruction::bytes(81, 31 + 128)));
        assert_eq!(commands.next(), None);
    }

    #[test]
    fn test_carriage_return_coordinates() {
        let symbol = Symbol::cr();
        let base_pos: Position = Default::default();
        let mut pos = base_pos.clone();
        // emulate the motion result caused by printing of 10 characters
        // causing the carriage to move by X=+120 units, when Y=0
        for _ in 0..10 {
            pos.jump(&pos.step_right());
        }
        assert_eq!(pos.diff(&base_pos), (120, 0));

        let mut pos: Position = Default::default();
        let base_pos: Position = Default::default();
        let action: Action = Action::new(symbol, Default::default());
        let _ = action.instructions(&base_pos, &mut pos);

        // The distance between the base point should be only
        // relevant in the +Y direction
        assert_eq!(pos.diff(&base_pos), (0, 16));
    }

    #[test]
    fn test_carriage_return_instructions() {
        let symbol = Symbol::cr();
        let base_pos: Position = Default::default();
        let mut pos = base_pos.clone();
        for _ in 0..10 {
            pos.jump(&pos.step_right());
        }

        let action = Action::new(symbol, Default::default());
        let mut cmd = action.instructions(&base_pos, &mut pos);

        assert_eq!(cmd.next(), Some(Instruction::Idle(SHORT_MS)));
        assert_eq!(cmd.next(), Some(Instruction::bytes(0b1110_0000, 120)));
        assert_eq!(cmd.next(), Some(Instruction::Idle(LONG_MS)));
        assert_eq!(cmd.next(), Some(Instruction::Idle(SHORT_MS)));
        assert_eq!(cmd.next(), Some(Instruction::bytes(0b1101_0000, 16)));
        assert_eq!(cmd.next(), Some(Instruction::Idle(LONG_MS)));
        assert_eq!(cmd.next(), None);
    }
}
