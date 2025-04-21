use std::str::Chars;

pub fn expand_escape(chars: &mut Chars<'_>) -> Option<char> {
    match chars.next()? {
        // Classiques
        '\\' => Some('\\'),
        'a' => Some('\x07'),
        'b' => Some('\x08'),
        'f' => Some('\x0C'),
        'n' => Some('\n'),
        'r' => Some('\r'),
        't' => Some('\t'),
        'v' => Some('\x0B'),

        // Octal : \[0-7]{1,3}
        c @ '0'..='7' => {
            let mut value = c.to_digit(8).unwrap();
            for _ in 0..2 {
                if let Some(peek) = chars.clone().next() {
                    if let Some(d) = peek.to_digit(8) {
                        chars.next(); // consomme
                        value = value * 8 + d;
                    } else {
                        break;
                    }
                }
            }
            std::char::from_u32(value)
        }

        // Hexadecimal : \x[0-9A-Fa-f]+
        'x' => {
            let mut value = 0;
            let mut found = false;
            while let Some(peek) = chars.clone().next() {
                if let Some(d) = peek.to_digit(16) {
                    chars.next(); // consomme
                    value = value * 16 + d;
                    found = true;
                } else {
                    break;
                }
            }
            if found {
                std::char::from_u32(value)
            } else {
                Some('x') // pas de chiffre après \x => devient 'x'
            }
        }

        // Tout autre caractère : retour inchangé
        c => Some(c),
    }
}
