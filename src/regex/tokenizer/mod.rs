mod charset;
mod concatenation;
mod postfix;
mod quantifier;
mod quotes;
pub mod tokenizer;
use charset::{expand_dot, extract_charset};
use quantifier::extract_repetition_range;
pub use tokenizer::*;

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
