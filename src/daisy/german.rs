use super::{Impact, Symbol};

impl Symbol {
    #[allow(unused)]
    fn new(idx: u8, chr: char) -> Self {
        Self {
            idx,
            chr,
            imp: Default::default(),
        }
    }

    #[allow(unused)]
    fn imp(mut self, impact: Impact) -> Self {
        self.imp = impact;
        self
    }
}

#[allow(unused)]
pub struct Db {
    symbols: [Symbol; 100],
    unknown: Symbol,
}

impl Db {
    pub fn new() -> Self {
        let symbols = [
            Symbol::new(1, '.').imp(Impact::Soft),
            Symbol::new(2, ',').imp(Impact::Soft),
            Symbol::new(3, '-').imp(Impact::Soft),
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
            Symbol::new(14, '´'),
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
            Symbol::new(50, 'W').imp(Impact::Hard),
            Symbol::new(51, '_'),
            Symbol::new(52, '='),
            Symbol::new(53, ';'),
            Symbol::new(54, ':'),
            Symbol::new(55, 'M').imp(Impact::Hard),
            Symbol::new(56, '\''),
            Symbol::new(57, 'H'),
            Symbol::new(58, '('),
            Symbol::new(59, 'K'),
            Symbol::new(60, '/'),
            Symbol::new(61, 'O').imp(Impact::Hard),
            Symbol::new(62, '!'),
            Symbol::new(63, 'X'),
            Symbol::new(64, '§').imp(Impact::Hard),
            Symbol::new(65, 'Q').imp(Impact::Hard),
            Symbol::new(66, 'J'),
            Symbol::new(67, '%'),
            Symbol::new(68, '³'), // U+00B3
            Symbol::new(69, 'G'),
            Symbol::new(70, '°'),
            Symbol::new(71, 'Ü').imp(Impact::Hard),
            Symbol::new(72, '`').imp(Impact::Soft),
            Symbol::new(73, 'Ö'),
            Symbol::new(74, '<'),
            Symbol::new(75, 'Ä').imp(Impact::Hard),
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
        ];
        let unknown = Symbol::new(41, '*');
        Self { symbols, unknown }
    }

    #[allow(unused)]
    pub fn get(&self, character: char) -> &Symbol {
        if let Some(result) = self.symbols.iter().find(|symbol| symbol.chr == character) {
            return result;
        }
        &(self.unknown)
    }
}
