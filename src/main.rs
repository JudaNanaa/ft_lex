// struct Test {
// 	x: i32,
// 	y: i32,
// }

// fn ok(test: Test) -> ()
// {
// 	println!("Hello, world! x {} and y {}", test.x, test.y);
// 	if test.x > test.y
// 	{
// 		println!("x: {} is bigger than y: {}", test.x, test.y);
// 	}
// 	else if test.x == test.y {
// 		println!("x: {} is equal to y: {}", test.x, test.y);
// 	}
// 	else {
// 		println!("y: {} is bigger than x: {}", test.y, test.x);
// 	}
// }

mod regex;
const REGEX: &str = "abc";

fn main() {
	let mut input = String::new();
	let result = std::io::stdin().read_line(&mut input);

	if result.is_err() {
		println!("error input");
	}
	else {
		println!("tu as tapee {}", input);
	}

	let list = regex::create_regex_list(REGEX.to_string());

	println!("list = {:?}", list);

	// let test = Test {
	// 	x: 15,
	// 	y: 86,
	// };
	// ok(test);
}
