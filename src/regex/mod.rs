use std::collections::LinkedList;
mod tokenizer;

#[derive(Debug, Clone)]
pub enum RegexAST {
    Char(char),
    Concat(Box<RegexAST>, Box<RegexAST>),
    Or(Box<RegexAST>, Box<RegexAST>),
    Star(Box<RegexAST>),
    Empty,
}

pub fn create_regex_ast(regex: &str) -> Result<RegexAST, &'static str>
{
	let tokens = tokenizer::regex_tokenizer(regex);
	dbg!(tokens);
	todo!("create_regex_ast");
}