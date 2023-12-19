#![allow(unused)]
#![allow(dead_code)]

use crate::daisy::{ActionMapping, Symbol};

/// The basic directive for the machine
/// Idle specifies milliseconds
/// SendBytes specifies a sequence of 2 bytes to be send over a serial port
///
#[derive(PartialEq, Debug)]
pub enum Instruction {
    #[allow(unused)]
    Idle(u64),
    SendBytes([u8; 2]),
}

impl Instruction {
    fn bytes(b1: u8, b2: u8) -> Self {
        Self::SendBytes([b1, b2])
    }
}

/// Action defines what we do with a Symbol
/// each instance of Action can produce a series of Instructions.
///
/// If a concrete Action cannot be performed in the concrete conditions
/// then the execution of program should be stopped.
///
// In the future releases the Action should be able to respond with a recovery option.
// For example a print-symbol action: when the carriage has reached the left limit,
// we cannot perform this action. The recovery-action would the carriage return.
// In this case a recovery strategy would look like this:
// - the current action (printing a symbol) cannot be performed;
// - tt responds with empty set (iterator) for instructions method;
// - the executor party checks for a recovery actions (should be proposed by the current one)
// - the recovery sequence would contain a copy of the current action at the end of the list
pub enum Action {
    BackSpace,
    Space,
    PrintSymbol(Symbol, Option<u16>),
    CarriageReturn,
}

impl From<Symbol> for Action {
    fn from(symbol: Symbol) -> Self {
        match symbol.act {
            ActionMapping::Whitespace => Self::Space,
            ActionMapping::Print => Self::PrintSymbol(symbol, None),
            ActionMapping::CarriageReturn => Self::CarriageReturn,
        }
    }
}

impl Action {
    pub fn instructions(self) -> impl Iterator<Item = Instruction> {
        match self {
            Self::PrintSymbol(symbol, repeat) => print_symbols(symbol, repeat),
            Self::Space => space_jump_right(),
            Self::BackSpace => space_jump_left(),
            Self::CarriageReturn => empty_command(),
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

pub fn print_symbols(symbol: Symbol, repeat: Option<u16>) -> Box<dyn Iterator<Item = Instruction>> {
    let times = repeat.unwrap_or(1);
    Box::new((0..times).map(move |_| print_single_symbol(&symbol)))
}

#[cfg(test)]
mod tests {
    use super::Action;
    use crate::daisy::Symbol;
    use crate::gabi::printing::Instruction;

    #[test]
    fn test_print_symbols_iterates_over_repeating_symbol() {
        let symbol = Symbol::new(81, 'ü');
        let action = Action::PrintSymbol(symbol, Some(2));
        let mut commands = action.instructions();
        assert_eq!(commands.next(), Some(Instruction::bytes(81, 0x96)));
        assert_eq!(commands.next(), Some(Instruction::bytes(81, 0x96)));
        assert_eq!(commands.next(), None);
    }

    #[test]
    fn test_print_symbol_once() {
        let symbol = Symbol::new(81, 'ü');
        let action = Action::PrintSymbol(symbol, None);
        let mut commands = action.instructions();
        assert_eq!(commands.next(), Some(Instruction::bytes(81, 0x96)));
        assert_eq!(commands.next(), None);
    }
}
