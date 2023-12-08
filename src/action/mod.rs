#![allow(unused)]
#![allow(dead_code)]

pub mod printing;

use crate::daisy::Symbol;
#[derive(PartialEq, Debug)]
pub struct Command(u8, u8);
pub enum Action {
    BackSpace,
    Space,
    PrintSymbol(Symbol, Option<u16>),
}

impl Action {
    fn commands(self) -> impl Iterator<Item = Command> {
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
