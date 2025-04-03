mod charset;
mod concatenation;
mod postfix;
mod quantifier;
mod quotes;
pub mod tokenizer;
mod utils;
use charset::*;
use quantifier::*;
pub use tokenizer::*;
use utils::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Or,
    TrailingContent,
    OpenGroup,
    CloseGroup,
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