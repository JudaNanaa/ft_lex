use crate::regex::utils::expand_escape;

use super::Operator::*;
use super::Quantifier::*;
use super::Token::Operator;
use super::{
    concatenation::add_concatenation_token, postfix::to_postfix, quotes::get_string_under_quotes, *,
};
use std::str::Chars;

const WHITESPACE: &str = " \n\r\t";

pub fn regex_tokenizer(regex: &str) -> (Vec<Token>, usize) {
    let mut token_list: Vec<Token> = Vec::new();
    let mut chars: Chars<'_> = regex.chars();
    let mut index = 0;

    while let Some(current_char) = chars.next() {
        match current_char {
            '"' => {
                let mut string_tokens = get_string_under_quotes(&mut chars, '"');
                token_list.append(&mut string_tokens);
            }
            '\\' => {
                if let Some(escaped_char) = chars.next() {
                    token_list.push(Token::Char(expand_escape(escaped_char)));
                } else {
                    token_list.push(Token::Char('\\'));
                }
            }
            '[' => {
                let mut charset_tokens = extract_charset(&mut chars);
                token_list.append(&mut charset_tokens);
            }
            '{' => {
                let quantifier = extract_repetition_range(&mut chars);
                token_list.push(Operator(Quantifier(quantifier)));
            }
            '(' => {
                token_list.push(Operator(OpenParen));
            }
            ')' => {
                token_list.push(Operator(CloseParen));
            }
            '|' => token_list.push(Operator(Or)),
            '/' => token_list.push(Operator(TrailingContent)),
            '?' => token_list.push(Operator(Quantifier(Range(0, 1)))),
            '*' => token_list.push(Operator(Quantifier(AtLeast(0)))),
            '+' => token_list.push(Operator(Quantifier(AtLeast(1)))),
            '.' => {
                let mut charset_tokens = expand_dot();
                token_list.append(&mut charset_tokens);
            }
            c => {
                if WHITESPACE.contains(c) {
                    break;
                }
                token_list.push(Token::Char(current_char))
            }
        }
        index += 1;
    }
    token_list = add_concatenation_token(token_list);
    token_list = to_postfix(token_list);
    return (token_list, index);
}

#[cfg(test)]
mod tests {
    use super::Token::{Char, Operator};
    use super::*;

    fn tok(regex: &str) -> Vec<Token> {
        let (token_list, _) = regex_tokenizer(&regex.to_string());
        return token_list;
    }

    #[test]
    fn test_simple_string() {
        let result = tok("abc");
        let expected = vec![
            Char('a'),
            Char('b'),
            Operator(Concatenation),
            Char('c'),
            Operator(Concatenation),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_string_in_quotes() {
        let result = tok("\"abc\"");
        let expected = vec![
            Char('a'),
            Char('b'),
            Operator(Concatenation),
            Char('c'),
            Operator(Concatenation),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_escape_sequence() {
        let result = tok("\\n");
        let expected = vec![Char('\n')];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_charset() {
        let result = tok("[abc]");
        let expected = vec![Char('a'), Char('b'), Operator(Or), Char('c'), Operator(Or)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_quantifiers() {
        let result = tok("a{3}");
        let expected = vec![Char('a'), Operator(Quantifier(Equal(3)))];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_range_quantifier() {
        let result = tok("a{2,5}");
        let expected = vec![Char('a'), Operator(Quantifier(Range(2, 5)))];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_question_star_plus() {
        let result = tok("a?b*c+");
        let expected = vec![
            Char('a'),
            Operator(Quantifier(Range(0, 1))),
            Char('b'),
            Operator(Quantifier(AtLeast(0))),
            Operator(Concatenation),
            Char('c'),
            Operator(Quantifier(AtLeast(1))),
            Operator(Concatenation),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_dot_operator() {
        let result = tok("a.b");
        assert!(result.len() > 3); // car . se développe en plein de OR
        assert_eq!(result[0], Char('a'));
        assert!(matches!(result[1], Char(_))); // un caractère de .
        assert!(matches!(result[2], Char(_))); // un caractère de .
        assert_eq!(result[3], Operator(Or));
    }

    #[test]
    fn test_grouping_and_or() {
        let result = tok("(a|b)c");
        assert_eq!(
            result,
            vec![
                Char('a'),
                Char('b'),
                Operator(Or),
                Char('c'),
                Operator(Concatenation),
            ]
        );
    }

    #[test]
    fn test_trailing_content() {
        let result = tok("abc/");
        assert_eq!(
            result,
            vec![
                Char('a'),
                Char('b'),
                Operator(Concatenation),
                Char('c'),
                Operator(Concatenation),
                Operator(TrailingContent),
            ]
        );
    }

    #[test]
    #[should_panic(expected = "Unclose quotes")]
    fn test_unclosed_quotes() {
        tok("\"abc");
    }

    #[test]
    fn test_complex_pattern() {
        let result = tok("(ab|cd)*e+f{2,}");
        let expected = vec![
            // (ab|cd)*
            Char('a'),
            Char('b'),
            Operator(Concatenation),
            Char('c'),
            Char('d'),
            Operator(Concatenation),
            Operator(Or),
            Operator(Quantifier(AtLeast(0))),
            // e+
            Char('e'),
            Operator(Quantifier(AtLeast(1))),
            Operator(Concatenation),
            // f{2,}
            Char('f'),
            Operator(Quantifier(AtLeast(2))),
            Operator(Concatenation),
        ];
        assert_eq!(result, expected);
    }
}
