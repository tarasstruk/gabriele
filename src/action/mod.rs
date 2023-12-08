#![allow(unused)]
#![allow(dead_code)]

pub mod printing;

use crate::daisy::{ActionMapping, Symbol};
use crate::gabi::commando::Commands;

#[derive(PartialEq, Debug)]
pub struct Command(u8, u8);

impl Command {
    pub fn to_bytes(&self) -> [u8; 2] {
        [self.0, self.1]
    }
}

pub enum Action {
    BackSpace,
    Space,
    PrintSymbol(Symbol, Option<u16>),
}

impl From<Symbol> for Action {
    fn from(symbol: Symbol) -> Self {
        match symbol.act {
            ActionMapping::Whitespace => Self::Space,
            ActionMapping::Print => Self::PrintSymbol(symbol, None),
        }
    }
}

impl Action {
    pub fn commands(self) -> impl Iterator<Item = Command> {
        match self {
            Self::PrintSymbol(symbol, repeat) => printing::print_symbols(symbol, repeat),
            Self::Space => space_jump_right(),
            Self::BackSpace => space_jump_left(),
        }
    }
}

fn space_jump_left() -> Box<dyn Iterator<Item = Command>> {
    Box::new([Command(0b1000_0100, 0b0000_0000)].into_iter())
}

fn space_jump_right() -> Box<dyn Iterator<Item = Command>> {
    Box::new([Command(0b1000_0011, 0b0000_0000)].into_iter())
}
