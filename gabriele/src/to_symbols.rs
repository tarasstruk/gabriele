use crate::database::DaisyDatabase;
use crate::symbol::Symbol;
use itertools::Itertools;

pub trait ToSymbols {
    fn to_symbols(&self, db: impl DaisyDatabase + 'static) -> impl Iterator<Item = Symbol>;
}

impl ToSymbols for &str {
    fn to_symbols(&self, db: impl DaisyDatabase + 'static) -> impl Iterator<Item = Symbol> {
        self.chars()
            .dedup_with_count()
            .map(move |(count, chr)| db.get(chr, count))
    }
}
