use super::Symbol;

pub fn symbols() -> Box<[Symbol]> {
    let things = [
        Symbol::new(1, '.').soft(),
        Symbol::new(2, ',').soft(),
        Symbol::new(3, '-').soft(),
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
        Symbol::new(50, 'W').hard(),
        Symbol::new(51, '_'),
        Symbol::new(52, '='),
        Symbol::new(53, ';'),
        Symbol::new(54, ':'),
        Symbol::new(55, 'M').hard(),
        Symbol::new(56, '\''),
        Symbol::new(57, 'H'),
        Symbol::new(58, '('),
        Symbol::new(59, 'K'),
        Symbol::new(60, '/'),
        Symbol::new(61, 'O').hard(),
        Symbol::new(62, '!'),
        Symbol::new(63, 'X'),
        Symbol::new(64, '§').hard(),
        Symbol::new(65, 'Q').hard(),
        Symbol::new(66, 'J'),
        Symbol::new(67, '%'),
        Symbol::new(68, '³'), // U+00B3
        Symbol::new(69, 'G'),
        Symbol::new(70, '°'),
        Symbol::new(71, 'Ü').hard(),
        Symbol::new(72, '`').soft(),
        Symbol::new(73, 'Ö'),
        Symbol::new(74, '<'),
        Symbol::new(75, 'Ä').hard(),
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
    Box::new(things)
}

#[cfg(test)]
mod tests {
    use crate::daisy::*;

    #[test]
    fn test_string_to_iterator_over_symbols() {
        let db = Db::new(super::symbols());

        let input = "Wombat";
        let mut first_iterator = db.printables(input);

        let mut second_iterator = db.printables(input);

        let sym_w_upper = Symbol::new(50, 'W').hard();

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
