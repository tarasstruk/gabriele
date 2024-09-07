use crate::daisy::Symbol;

#[allow(unused)]
pub struct Db {
    pub symbols: Box<[Symbol]>,
    pub unknown: Symbol,
}

fn symbols() -> Box<[Symbol]> {
    let things = [
        Symbol::new('.').petal(1).mild(),
        Symbol::new(',').petal(2).mild(),
        Symbol::new('-').petal(3).mild(),
        Symbol::new('v').petal(4),
        Symbol::new('l').petal(5),
        Symbol::new('m').petal(6),
        Symbol::new('j').petal(7),
        Symbol::new('w').petal(8),
        Symbol::new('²').petal(9), // U+00B2
        Symbol::new('µ').petal(10),
        Symbol::new('f').petal(11),
        Symbol::new('^').petal(12),
        Symbol::new('>').petal(13),
        Symbol::new('´').petal(14), // acute accent
        Symbol::new('’').petal(14), // a replacement for the apostrophe
        Symbol::new('+').petal(15),
        Symbol::new('1').petal(16),
        Symbol::new('2').petal(17),
        Symbol::new('3').petal(18),
        Symbol::new('4').petal(19),
        Symbol::new('5').petal(20),
        Symbol::new('6').petal(21),
        Symbol::new('7').petal(22),
        Symbol::new('8').petal(23),
        Symbol::new('9').petal(24),
        Symbol::new('0').petal(25),
        Symbol::new('E').petal(26),
        Symbol::new('|').petal(27),
        Symbol::new('B').petal(28),
        Symbol::new('F').petal(29),
        Symbol::new('P').petal(30),
        Symbol::new('S').petal(31),
        Symbol::new('Z').petal(32),
        Symbol::new('V').petal(33),
        Symbol::new('&').petal(34),
        Symbol::new('Y').petal(35),
        Symbol::new('A').petal(36),
        Symbol::new('T').petal(37),
        Symbol::new('L').petal(38),
        Symbol::new('$').petal(39),
        Symbol::new('R').petal(40),
        Symbol::new('*').petal(41),
        Symbol::new('C').petal(42),
        Symbol::new('"').petal(43),
        Symbol::new('D').petal(44),
        Symbol::new('?').petal(45),
        Symbol::new('N').petal(46),
        Symbol::new('I').petal(47),
        Symbol::new('U').petal(48),
        Symbol::new(')').petal(49),
        Symbol::new('W').petal(50).strong(),
        Symbol::new('_').petal(51),
        Symbol::new('=').petal(52),
        Symbol::new(';').petal(53),
        Symbol::new(':').petal(54),
        Symbol::new('M').petal(55).strong(),
        Symbol::new('\'').petal(56),
        Symbol::new('H').petal(57),
        Symbol::new('(').petal(58),
        Symbol::new('K').petal(59),
        Symbol::new('/').petal(60),
        Symbol::new('O').petal(61).strong(),
        Symbol::new('!').petal(62),
        Symbol::new('X').petal(63),
        Symbol::new('§').petal(64).strong(),
        Symbol::new('Q').petal(65).strong(),
        Symbol::new('J').petal(66),
        Symbol::new('%').petal(67),
        Symbol::new('³').petal(68), // U+00B3
        Symbol::new('G').petal(69),
        Symbol::new('°').petal(70),
        Symbol::new('Ü').petal(71).strong(),
        Symbol::new('`').petal(72).mild(), // grave accent
        Symbol::new('Ö').petal(73),
        Symbol::new('<').petal(74),
        Symbol::new('Ä').petal(75).strong(),
        Symbol::new('#').petal(76),
        Symbol::new('t').petal(77),
        Symbol::new('x').petal(78),
        Symbol::new('q').petal(79),
        Symbol::new('ß').petal(80),
        Symbol::new('ü').petal(81),
        Symbol::new('ö').petal(82),
        Symbol::new('ä').petal(83),
        Symbol::new('y').petal(84),
        Symbol::new('k').petal(85),
        Symbol::new('p').petal(86),
        Symbol::new('h').petal(87),
        Symbol::new('c').petal(88),
        Symbol::new('g').petal(89),
        Symbol::new('n').petal(90),
        Symbol::new('r').petal(91),
        Symbol::new('s').petal(92),
        Symbol::new('e').petal(93),
        Symbol::new('a').petal(94),
        Symbol::new('i').petal(95),
        Symbol::new('d').petal(96),
        Symbol::new('u').petal(97),
        Symbol::new('b').petal(98),
        Symbol::new('o').petal(99),
        Symbol::new('z').petal(100),
        Symbol::new('ù').petal(97).grave(),
        Symbol::new('Ù').petal(48).grave(),
        Symbol::new('è').petal(93).grave(),
        Symbol::new('È').petal(26).grave(),
        Symbol::new('ì').petal(95).grave(),
        Symbol::new('Ì').petal(47).grave(),
        Symbol::new('à').petal(94).grave(),
        Symbol::new('À').petal(36).grave().strong(),
        Symbol::new('ò').petal(99).grave(),
        Symbol::new('Ò').petal(61).grave().strong(),
        Symbol::new('ú').petal(97).acute(),
        Symbol::new('Ú').petal(48).acute(),
        Symbol::new('é').petal(93).acute(),
        Symbol::new('É').petal(26).acute(),
        Symbol::new('í').petal(95).acute(),
        Symbol::new('Í').petal(47).acute(),
        Symbol::new('á').petal(94).acute(),
        Symbol::new('Á').petal(36).acute().strong(),
        Symbol::new('ó').petal(99).acute(),
        Symbol::new('Ó').petal(61).acute().strong(),
        Symbol::whitespace(),
        Symbol::cr(),
    ];
    Box::new(things)
}

impl Default for Db {
    fn default() -> Self {
        Self {
            symbols: symbols(),
            unknown: Symbol::new('*').petal(41),
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

        let sym_w_upper = Symbol::new('W').petal(50).strong();

        let sym_o = Symbol::new('o').petal(99);

        let sym_m = Symbol::new('m').petal(6);

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
