use crate::database::DaisyDatabase;
use crate::symbol::Symbol;
use itertools::Itertools;

pub trait ToSymbols {
    fn to_symbols<'a>(&'a self, db: impl DaisyDatabase + 'a) -> impl Iterator<Item = Symbol> + 'a;
}

impl ToSymbols for &str {
    fn to_symbols<'a>(&'a self, db: impl DaisyDatabase + 'a) -> impl Iterator<Item = Symbol> + 'a {
        self.chars()
            .dedup_with_count()
            .map(move |(count, chr)| db.get(chr, count))
    }
}
