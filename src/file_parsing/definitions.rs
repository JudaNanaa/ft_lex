
use super::FileInfo;

const WHITESPACE: &str = " \r\t";


#[derive(Debug)]
pub enum DefinitionToken {
	Bloc{
		content: String,
	},
	LineWithSpace {
		content: String,
	},
	Definition{
		name: String,
		value: String
	},

}

fn get_content_under_brace(file: &mut FileInfo) -> Result<String, String> {
	let mut content = String::new();

	while let Some(char) = file.it.next() {
		match char {
			'%' => {
				if let Some(char_next) = file.it.peek() {
					if *char_next == '}' {
						file.it.next();
						return Ok(content);
					}
				}
			}
			c => {
				if c == '\n' {
					file.line_nb += 1;
				}
				content.push(c);
			},
		}
	}
	return Err(format!("{}:{}: premature EOF", file.name, file.line_nb));
}

fn get_line_with_space(file: &mut FileInfo) -> String {
	let mut content = String::new();

	while let Some(char) = file.it.next() {
		if char == '\n' {
			file.line_nb += 1;
			break;
		}
		content.push(char);
	}
	return content;
}

fn get_definition(first_letter: char, file: &mut FileInfo) -> Result<DefinitionToken, String> {
	let mut line = String::new();

	line.push(first_letter);
	while let Some(char) = file.it.next() {
		if char == '\n' {
			file.line_nb += 1;
			break;
		}
		line.push(char);
	}

	line = line.trim().to_string();
	match line.find(' ') {
		Some(index) => {
			let name = line[0..index].to_string();
			let value = line[index+1..].trim().to_string();
			return Ok(DefinitionToken::Definition {
				name, 
				value
			});
		}, 
		None => {
			return Err("incomplete name definition".to_string());
		}
	}
}

pub fn parse_definitions_part(file: &mut FileInfo) -> Result<Vec<DefinitionToken>, String> {
	let mut definition_list = Vec::new();
	while let Some(char) = file.it.next() {
		match char {
			'%' => {
				if let Some(char_next) = file.it.peek() {
					if *char_next == '{' {
						file.it.next();
						let content = get_content_under_brace(file)?;
						definition_list.push(DefinitionToken::Bloc{
							content
						});
					}
					else if *char_next == '%' {
						dbg!(&definition_list);
						return Ok(definition_list);
					}
				}
			}
			'\n' => file.line_nb += 1,
			c => {
				if WHITESPACE.contains(c) {
					let content = get_line_with_space(file);
					definition_list.push(DefinitionToken::LineWithSpace {
						content
					});
				}
				else {
					let definition_token = get_definition(c, file)?;
					definition_list.push(definition_token);

				}
			},
		}
	}
	return Err(format!("{}:{}: premature EOF", file.name, file.line_nb));
}