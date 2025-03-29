mod regex;

fn main() {
	let mut input = String::new();
	let result = std::io::stdin().read_line(&mut input);

	if result.is_err() {
		println!("error input");
	}
	else {
		println!("tu as tapee {}", input);
	}

	// Tokenizer
	let mut tokens = regex::tokenizer::regex_tokenizer(&input);
	// tokens = dbg!(tokens);
	// Parsing
	regex::parsing::regex_parsing(tokens);
	

	// AST


}
