use std::str::Chars;

pub trait VecUtils<T>
where
    T: PartialEq,
{
    fn remove_element(&mut self, element: &T) -> Option<T>;
    fn push_unique(&mut self, element: T);
}

impl<T> VecUtils<T> for Vec<T>
where
    T: PartialEq + std::cmp::Ord,
{
    fn remove_element(&mut self, element: &T) -> Option<T> {
        self.iter()
            .position(|e| e == element)
            .map(|index| self.remove(index))
    }

    fn push_unique(&mut self, element: T) {
        if !self.contains(&element) {
            self.push(element);
        }
    }
}

// pub fn expand_escape(c: char) -> char {
// 	match c {
//         '\\' => return '\\',
//         'a' => return '\x07', // bell
//         'b' => return '\x08', // backspace
//         'f' => return '\x0C', // formfeed
//         'n' => return '\n',   // newline
//         'r' => return '\r',   // carriage return
//         't' => return '\t',   // horizontal tab
//         'v' => return '\x0B', // vertical tab
//         _ => return c, // Not a recognized escape
//     }
// }

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
