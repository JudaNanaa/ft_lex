use std::char;

use super::{DefinitionToken, FileInfo};

const WHITESPACE: &str = " \r\t";

#[derive(Debug)]
enum DefinitionState {
    Inclusive,
    Exclusive,
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
            }
        }
    }
    return Err("premature EOF".to_string());
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
            let value = line[index + 1..].trim().to_string();
            return Ok(DefinitionToken::Definition { name, value });
        }
        None => {
            return Err("incomplete name definition".to_string());
        }
    }
}

fn split(str: &String, charser: &String) -> Vec<String> {
    let mut split = Vec::new();

    let mut str_it = str.chars();

    let mut current_str = String::new();

    while let Some(char) = str_it.next() {
        if charser.contains(char) {
            if !current_str.is_empty() {
                split.push(current_str.clone());
                current_str.clear();
            }
        } else {
            current_str.push(char);
        }
    }
    if !current_str.is_empty() {
        split.push(current_str.clone());
        current_str.clear();
    }
    return split;
}

fn get_state(file: &mut FileInfo) -> Result<Vec<String>, String> {
    let mut content = String::new();

    file.it.next();
    while let Some(char) = file.it.next() {
        match char {
            '\n' => {
                file.line_nb += 1;
                let split = split(&content, &" \t".to_string());
                if split.len() == 0 {
                    return Err("bad start condition list".to_string());
                }
                return Ok(split);
            }
            c => {
                content.push(c);
            }
        }
    }
    return Err("Premature EOF".to_string());
}

pub fn parse_definitions_part(file: &mut FileInfo) -> Result<Vec<DefinitionToken>, String> {
    let mut definition_list = Vec::new();
    let mut inclusive_states = Vec::new();
    let mut exclusive_states = Vec::new();
    while let Some(char) = file.it.next() {
        match char {
            '%' => {
                if let Some(char_next) = file.it.peek() {
                    if *char_next == '{' {
                        file.it.next();
                        let content = get_content_under_brace(file)?;
                        definition_list.push(DefinitionToken::Bloc { content });
                    } else if *char_next == '%' {
                        definition_list.push(DefinitionToken::InclusiveState {
                            names: inclusive_states,
                        });
                        definition_list.push(DefinitionToken::ExclusiveState {
                            names: exclusive_states,
                        });
                        dbg!(&definition_list);
                        return Ok(definition_list);
                    } else if *char_next == 's' || *char_next == 'x' {
                        let state = match *char_next {
                            's' => DefinitionState::Inclusive,
                            'x' => DefinitionState::Exclusive,
                            _ => panic!("Not normal"),
                        };
                        let inclusive_tokens = get_state(file)?;
                        for elem in inclusive_tokens {
                            if inclusive_states.contains(&elem) || exclusive_states.contains(&elem)
                            {
                                eprintln!(
                                    "{}:{}: start condition {} declared twice",
                                    file.name, file.line_nb, elem
                                );
                            }
                            match state {
                                DefinitionState::Inclusive => inclusive_states.push(elem),
                                DefinitionState::Exclusive => exclusive_states.push(elem),
                            }
                        }
                    }
                }
            }
            '\n' => file.line_nb += 1,
            c => {
                if WHITESPACE.contains(c) {
                    let content = get_line_with_space(file);
                    definition_list.push(DefinitionToken::LineWithSpace { content });
                } else {
                    let definition_token = get_definition(c, file)?;
                    definition_list.push(definition_token);
                }
            }
        }
    }
    return Err("premature EOF".to_string());
}
