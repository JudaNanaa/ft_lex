use std::{f64::consts::E, fs::File, io::Read, iter::Peekable, str::Chars};

use crate::file_parsing::{rules::parse_rules_part, FileState};

use super::{definitions::parse_definitions_part, user_routine::parse_user_routine_part, FileInfo};

fn get_file_content(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
	let mut file = File::open(file_path)?;
	let mut file_content = String::new();
	
	file.read_to_string(&mut file_content)?;
	// println!("{}", file_content);
	
	return Ok(file_content);
}

// pub fn parsing_lex_file(file_path: &str) -> Result<(), String>  {
// 	let file_content = match get_file_content(file_path) {
// 		Ok(content) => content,
// 		Err(_) => return Err("Cannot open file".to_string()),
// 	};
// 	let line_split: Vec<&str> = file_content.split('\n').collect();
// 	let mut file_state = FileState::Definition;
// 	let mut line_nb = 0;

// 	let mut line_it = line_split.iter().peekable();
// 	while let Some(line) = line_it.next() {
// 		line_nb += 1;
// 		if line.starts_with("%%") {
// 			match file_state {
// 				FileState::Definition => {
// 					file_state = FileState::Rules;
// 					continue;
// 				},
// 				FileState::Rules => {
// 					file_state = FileState::UserRoutine;
// 					continue;
// 				},
// 				_ => {},
// 			}
// 		}
// 		match file_state {
// 			FileState::Definition => {
// 				parse_definitions_part(line, &mut line_it);
// 			},
// 			FileState::Rules => {
// 				parse_rules_part(line, &mut line_it);
// 				println!("{}", line);
// 			},
// 			FileState::UserRoutine => {
				
// 			},
// 		}
// 	}
// 	if file_state == FileState::Definition {
// 		let error = format!("{}:{}: premature EOF", file_path, line_nb);
// 		return Err(error);
// 	}
// 	return Ok(());
// }

pub fn parsing_lex_file(file_path: &str) -> Result<(), String>  {
	let file_content = match get_file_content(file_path) {
		Ok(content) => content,
		Err(_) => return Err("Cannot open file".to_string()),
	};
	let mut file = FileInfo {
		it: file_content.chars().peekable(),
		line_nb: 0,
		name: file_path
	};

	match parse_definitions_part(&mut file) {
		Ok(_) => {},
		Err(message) => {
			println!("{}:{}: {}", file.name, file.line_nb, message);
		}
	}
	// parse_rules_part(&mut file_content_it);
	// parse_user_routine_part(&mut file_content_it);
	return Ok(());
}