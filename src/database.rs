use crate::daisy::Symbol;

#[allow(unused)]
pub struct Db {
    pub symbols: Box<[Symbol]>,
    pub unknown: Symbol,
}

fn symbols() -> Box<[Symbol]> {
    let things = [
        Symbol::new(1, '.').mild(),
        Symbol::new(2, ',').mild(),
        Symbol::new(3, '-').mild(),
        Symbol::new(4, 'v'),
        Symbol::new(5, 'l'),
        Symbol::new(6, 'm'),
        Symbol::new(7, 'j'),
        Symbol::new(8, 'w'),
        Symbol::new(9, '²'), // U+00B2
        Symbol::new(10, 'µ'),
        Symbol::new(11, 'f'),
        Symbol::new(12, '^'),
        Symbol::new(13, '>'),
        Symbol::new(14, '´'), // acute accent
        Symbol::new(14, '’'), // a replacement for the apostrophe
        Symbol::new(15, '+'),
        Symbol::new(16, '1'),
        Symbol::new(17, '2'),
        Symbol::new(18, '3'),
        Symbol::new(19, '4'),
        Symbol::new(20, '5'),
        Symbol::new(21, '6'),
        Symbol::new(22, '7'),
        Symbol::new(23, '8'),
        Symbol::new(24, '9'),
        Symbol::new(25, '0'),
        Symbol::new(26, 'E'),
        Symbol::new(27, '|'),
        Symbol::new(28, 'B'),
        Symbol::new(29, 'F'),
        Symbol::new(30, 'P'),
        Symbol::new(31, 'S'),
        Symbol::new(32, 'Z'),
        Symbol::new(33, 'V'),
        Symbol::new(34, '&'),
        Symbol::new(35, 'Y'),
        Symbol::new(36, 'A'),
        Symbol::new(37, 'T'),
        Symbol::new(38, 'L'),
        Symbol::new(39, '$'),
        Symbol::new(40, 'R'),
        Symbol::new(41, '*'),
        Symbol::new(42, 'C'),
        Symbol::new(43, '"'),
        Symbol::new(44, 'D'),
        Symbol::new(45, '?'),
        Symbol::new(46, 'N'),
        Symbol::new(47, 'I'),
        Symbol::new(48, 'U'),
        Symbol::new(49, ')'),
        Symbol::new(50, 'W').strong(),
        Symbol::new(51, '_'),
        Symbol::new(52, '='),
        Symbol::new(53, ';'),
        Symbol::new(54, ':'),
        Symbol::new(55, 'M').strong(),
        Symbol::new(56, '\''),
        Symbol::new(57, 'H'),
        Symbol::new(58, '('),
        Symbol::new(59, 'K'),
        Symbol::new(60, '/'),
        Symbol::new(61, 'O').strong(),
        Symbol::new(62, '!'),
        Symbol::new(63, 'X'),
        Symbol::new(64, '§').strong(),
        Symbol::new(65, 'Q').strong(),
        Symbol::new(66, 'J'),
        Symbol::new(67, '%'),
        Symbol::new(68, '³'), // U+00B3
        Symbol::new(69, 'G'),
        Symbol::new(70, '°'),
        Symbol::new(71, 'Ü').strong(),
        Symbol::new(72, '`').mild(), // grave accent
        Symbol::new(73, 'Ö'),
        Symbol::new(74, '<'),
        Symbol::new(75, 'Ä').strong(),
        Symbol::new(76, '#'),
        Symbol::new(77, 't'),
        Symbol::new(78, 'x'),
        Symbol::new(79, 'q'),
        Symbol::new(80, 'ß'),
        Symbol::new(81, 'ü'),
        Symbol::new(82, 'ö'),
        Symbol::new(83, 'ä'),
        Symbol::new(84, 'y'),
        Symbol::new(85, 'k'),
        Symbol::new(86, 'p'),
        Symbol::new(87, 'h'),
        Symbol::new(88, 'c'),
        Symbol::new(89, 'g'),
        Symbol::new(90, 'n'),
        Symbol::new(91, 'r'),
        Symbol::new(92, 's'),
        Symbol::new(93, 'e'),
        Symbol::new(94, 'a'),
        Symbol::new(95, 'i'),
        Symbol::new(96, 'd'),
        Symbol::new(97, 'u'),
        Symbol::new(98, 'b'),
        Symbol::new(99, 'o'),
        Symbol::new(100, 'z'),
        Symbol::new(97, 'ù').grave(),
        Symbol::new(48, 'Ù').grave(),
        Symbol::new(93, 'è').grave(),
        Symbol::new(26, 'È').grave(),
        Symbol::new(95, 'ì').grave(),
        Symbol::new(47, 'Ì').grave(),
        Symbol::new(94, 'à').grave(),
        Symbol::new(36, 'À').grave().strong(),
        Symbol::new(99, 'ò').grave(),
        Symbol::new(61, 'Ò').grave().strong(),
        Symbol::new(97, 'ú').acute(),
        Symbol::new(48, 'Ú').acute(),
        Symbol::new(93, 'é').acute(),
        Symbol::new(26, 'É').acute(),
        Symbol::new(95, 'í').acute(),
        Symbol::new(47, 'Í').acute(),
        Symbol::new(94, 'á').acute(),
        Symbol::new(36, 'Á').acute().strong(),
        Symbol::new(99, 'ó').acute(),
        Symbol::new(61, 'Ó').acute().strong(),
        Symbol::whitespace(),
        Symbol::cr(),
    ];
    Box::new(things)
}

impl Default for Db {
    fn default() -> Self {
        Self {
            symbols: symbols(),
            unknown: Symbol::new(41, '*'),
        }
    }
}

impl Db {
    #[allow(unused)]
    pub fn get(&self, character: char) -> &Symbol {
        if let Some(result) = self
            .symbols
            .iter()
            .find(|symbol| symbol.character == character)
        {
            return result;
        }
        &(self.unknown)
    }

    pub fn new() -> Self {
        Default::default()
    }

    pub fn printables<'a>(&'a self, input: &'a str) -> impl Iterator<Item = &'a Symbol> {
        input.chars().enumerate().map(move |(_, chr)| self.get(chr))
    }
}

#[cfg(test)]
mod tests {
    use super::Db;
    use crate::daisy::Symbol;

    #[test]
    fn test_string_to_iterator_over_symbols() {
        let db = Db::new();

        let input = "Wombat";
        let mut first_iterator = db.printables(input);

        let mut second_iterator = db.printables(input);

        let sym_w_upper = Symbol::new(50, 'W').strong();

        let sym_o = Symbol::new(99, 'o');

        let sym_m = Symbol::new(6, 'm');

        let value = first_iterator.next();
        assert_eq!(value, Some(&sym_w_upper));

        let value = first_iterator.next();
        assert_eq!(value, Some(&sym_o));

        let value = second_iterator.next();
        assert_eq!(value, Some(&sym_w_upper));

        let value = first_iterator.next();
        assert_eq!(value, Some(&sym_m));
    }
}
