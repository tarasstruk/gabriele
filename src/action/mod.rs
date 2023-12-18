#![allow(unused)]
#![allow(dead_code)]

pub mod printing;

use crate::daisy::{ActionMapping, Symbol};
use crate::gabi::primitives::Instruction;

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
            Self::PrintSymbol(symbol, repeat) => printing::print_symbols(symbol, repeat),
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
    Box::new([Instruction::SendBytes([0b1000_0100, 0b0000_0000])].into_iter())
}

fn space_jump_right() -> Box<dyn Iterator<Item = Instruction>> {
    Box::new([Instruction::SendBytes([0b1000_0011, 0b0000_0000])].into_iter())
}
