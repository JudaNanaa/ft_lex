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

	let list = regex::create_regex_ast(&input);

	println!("list = {:?}", list);
}
