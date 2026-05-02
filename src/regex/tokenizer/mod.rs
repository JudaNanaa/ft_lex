mod charset;
mod concatenation;
pub mod parse;
mod postfix;
mod quantifier;
mod quotes;
use charset::{expand_dot, extract_charset};
pub use parse::*;
use quantifier::extract_repetition_range;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Or,
    TrailingContext,
    OpenParen,
    CloseParen,
    Quantifier(Quantifier),
    Concatenation,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Char(char),
    Operator(Operator),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Quantifier {
    Equal(usize),
    AtLeast(usize),
    Range(usize, usize),
}
