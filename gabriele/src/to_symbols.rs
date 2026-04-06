use crate::database::DaisyDatabase;
use crate::symbol::Symbol;

pub trait ToSymbols {
    fn to_symbols(&self, db: impl DaisyDatabase + 'static) -> impl Iterator<Item = Symbol>;
}

impl ToSymbols for &str {
    fn to_symbols(&self, db: impl DaisyDatabase + 'static) -> impl Iterator<Item = Symbol> {
        self.chars().map(move |chr| db.get(chr))
    }
}
