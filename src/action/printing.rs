use crate::action::Command;
use crate::daisy::Symbol;

fn print_single_symbol(symbol: &Symbol) -> Command {
    Command(symbol.idx, 0b1001_0110)
}

pub fn print_symbols(symbol: Symbol, repeat: Option<u16>) -> Box<dyn Iterator<Item = Command>> {
    let times = repeat.unwrap_or(1);
    Box::new((0..times).map(move |_| print_single_symbol(&symbol)))
}

#[cfg(test)]
mod tests {
    use crate::action::{Action, Command};
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
