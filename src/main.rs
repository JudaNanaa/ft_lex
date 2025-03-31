mod regex;
use crate::regex::tokenizer::tokenizer::{RegexToken, regex_tokenizer};

fn main() {
	let mut input: String = String::new();
	let result: Result<usize, std::io::Error> = std::io::stdin().read_line(&mut input);

	if result.is_err() {
		println!("error input");
	}
	else {
		println!("tu as tapee {}", input);
	}

	// Tokenizer
	let mut tokens: Vec<RegexToken> = regex_tokenizer(&input);
	tokens = dbg!(tokens);
	
	// Parsing
	// regex::parsing::regex_parsing(tokens);
	

	// AST


}
