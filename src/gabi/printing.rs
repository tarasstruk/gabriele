#![allow(unused)]
#![allow(dead_code)]

use crate::daisy::{ActionMapping, Symbol};
use crate::gabi::motion;
use crate::gabi::position::Position;

/// The basic directive for the machine
/// Idle specifies milliseconds
/// SendBytes specifies a sequence of 2 bytes to be send over a serial port
#[derive(PartialEq, Debug)]
pub enum Instruction {
    #[allow(unused)]
    Idle(u64),
    SendBytes([u8; 2]),
    Empty,
}

impl Instruction {
    pub fn bytes(b1: u8, b2: u8) -> Self {
        Self::SendBytes([b1, b2])
    }
    pub fn wait_short() -> Self {
        Self::Idle(200)
    }

    pub fn wait_tiny() -> Self {
        Self::Idle(50)
    }

    pub fn wait_long() -> Self {
        Self::Idle(1000)
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
    pub base_position: Position,
    pub current_position: Position,
}

impl Action {
    pub fn new(symbol: Symbol, base_position: Position, current_position: Position) -> Self {
        Self {
            symbol,
            base_position,
            current_position,
        }
    }

    /// Generates a sequence of the Instructions,
    /// taking the current Position as a reference point.
    /// The result of these instructions is the printed Symbol or/and the associated motion.
    pub fn instructions(&self) -> impl Iterator<Item = Instruction> {
        match self.symbol.act {
            ActionMapping::Print => print_symbols(self.symbol.clone(), None),
            ActionMapping::Whitespace => space_jump_right(),
            ActionMapping::CarriageReturn => {
                let new_pos = self.current_position.cr(&self.base_position);
                motion::move_absolute(self.current_position.clone(), new_pos)
            }
        }
    }

    /// New position represents a calculated desired Position
    /// where the machine is expected to be
    /// after the generated Instructions have been executed.
    pub fn new_position(&self) -> Position {
        match self.symbol.act {
            ActionMapping::Print => self.current_position.step_right(),
            ActionMapping::Whitespace => self.current_position.step_right(),
            ActionMapping::CarriageReturn => self.current_position.cr(&self.base_position),
        }
    }
}

fn empty_command() -> Box<dyn Iterator<Item = Instruction>> {
    let cmd: [Instruction; 0] = [];
    Box::new(cmd.into_iter())
}

fn space_jump_left() -> Box<dyn Iterator<Item = Instruction>> {
    Box::new([Instruction::bytes(0b1000_0100, 0b0000_0000)].into_iter())
}

fn space_jump_right() -> Box<dyn Iterator<Item = Instruction>> {
    Box::new([Instruction::bytes(0b1000_0011, 0b0000_0000)].into_iter())
}

fn print_single_symbol(symbol: &Symbol) -> Instruction {
    Instruction::bytes(symbol.idx, 0b1001_0110)
}

fn print_symbols(symbol: Symbol, repeat: Option<u16>) -> Box<dyn Iterator<Item = Instruction>> {
    let times = repeat.unwrap_or(1);
    Box::new((0..times).map(move |_| print_single_symbol(&symbol)))
}

#[cfg(test)]
mod tests {
    use super::Action;
    use crate::daisy::Symbol;
    use crate::gabi::position::Position;
    use crate::gabi::printing::Instruction;

    #[test]
    fn test_print_symbol() {
        let symbol = Symbol::new(81, 'ü');
        let pos: Position = Default::default();
        let actor = Action::new(symbol, pos.clone(), pos.clone());
        let mut commands = actor.instructions();
        assert_eq!(commands.next(), Some(Instruction::bytes(81, 0x96)));
        assert_eq!(commands.next(), None);
    }
}
