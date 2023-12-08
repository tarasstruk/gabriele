#![allow(unused)]
#![allow(dead_code)]

use crate::daisy::Symbol;
#[derive(PartialEq, Debug)]
pub struct Command(u8, u8);

#[allow(dead_code)]
pub enum Action {
    BackSpace,
    Space,
    PrintSymbol(Symbol, Option<u16>),
}

impl Action {
    fn commands(self) -> impl Iterator<Item = Command> {
        match self {
            Self::PrintSymbol(symbol, repeat) => print_symbols(symbol, repeat),
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

fn print_single_symbol(symbol: &Symbol) -> Command {
    Command(symbol.idx, 0b1001_0110)
}

fn print_symbols(symbol: Symbol, repeat: Option<u16>) -> Box<dyn Iterator<Item = Command>> {
    let times = repeat.unwrap_or(1);
    Box::new((0..times).map(move |_| print_single_symbol(&symbol)))
}

#[cfg(test)]
mod tests {
    use super::{Action, Command};
    use crate::daisy::Symbol;

    #[test]
    fn test_print_symbols_iterates_over_repeating_symbol() {
        let symbol = Symbol::new(81, 'ü');
        let action = Action::PrintSymbol(symbol, Some(2));
        let mut commands = action.commands();
        assert_eq!(commands.next(), Some(Command(81, 0x96)));
        assert_eq!(commands.next(), Some(Command(81, 0x96)));
        assert_eq!(commands.next(), None);
    }

    #[test]
    fn test_print_symbol_once() {
        let symbol = Symbol::new(81, 'ü');
        let action = Action::PrintSymbol(symbol, None);
        let mut commands = action.commands();
        assert_eq!(commands.next(), Some(Command(81, 0x96)));
        assert_eq!(commands.next(), None);
    }
}
