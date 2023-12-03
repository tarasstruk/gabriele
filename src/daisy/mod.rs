mod german;
use std::default::Default;

#[derive(Default)]
#[repr(u16)]
#[allow(unused)]
pub enum Impact {
    #[default]
    Middle = 127,
    Hard = 255,
    Soft = 90,
    Custom(u16),
}

pub struct Symbol {
    pub idx: u8,
    pub imp: Impact,
    pub chr: char,
}

trait Printable {
    fn index(&self) -> u8;
}

impl Printable for Symbol {
    fn index(&self) -> u8 {
        self.idx
    }
}

#[allow(unused)]
pub fn database() -> german::Db {
    german::Db::new()
}

#[allow(unused)]
fn get_symbols<'a>(input: &'a str, db: &'a german::Db) -> impl Iterator<Item = u8> + 'a {
    input
        .chars()
        .enumerate()
        .map(move |(_, chr)| db.get(chr).index())
}

#[cfg(test)]
mod tests {
    use crate::daisy::*;

    #[test]
    fn test_string_to_iterator_over_symbols() {
        let db = database();
        let mut first_iterator = get_symbols("Wombat", &db);

        let mut second_iterator = get_symbols("Wombat", &db);
        let value = first_iterator.next();
        assert_eq!(value, Some(50_u8));

        let value = first_iterator.next();
        assert_eq!(value, Some(99_u8));

        let value = second_iterator.next();
        assert_eq!(value, Some(50_u8));

        let value = first_iterator.next();
        assert_eq!(value, Some(6_u8));
    }
}
