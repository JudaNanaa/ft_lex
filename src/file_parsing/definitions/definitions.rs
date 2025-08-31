use std::char;

use crate::file_parsing::{
    definitions::{Definition, DefinitionState},
    FileInfo,
};

const WHITESPACE: &str = " \r\t";

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

fn get_definition(first_letter: char, file: &mut FileInfo) -> Result<Definition, String> {
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
            return Ok(Definition::Definition { name, value });
        }
        None => {
            return Err("incomplete name definition".to_string());
        }
    }
}

fn split(str: &str, charser: &str) -> Vec<String> {
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
    for char in file.it.by_ref() {
        match char {
            '\n' => {
                file.line_nb += 1;
                let split = split(&content, &" \t".to_string());
                if split.is_empty() {
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

fn skip_to_newline(file: &mut FileInfo) {
    for char in file.it.by_ref() {
        if char == '\n' {
            file.line_nb += 1;
            return;
        }
    }
}

pub fn parse_definitions_part(file: &mut FileInfo) -> Result<Vec<Definition>, String> {
    let mut definition_list = Vec::new();
    let mut state_nb = 1;

    while let Some(char) = file.it.next() {
        match char {
            '%' => {
                if let Some(char_next) = file.it.peek() {
                    if *char_next == '{' {
                        file.it.next();
                        let content = get_content_under_brace(file)?;
                        definition_list.push(Definition::Bloc { content });
                    } else if *char_next == '%' {
                        skip_to_newline(file);
                        return Ok(definition_list);
                    } else if *char_next == 's' || *char_next == 'x' {
                        let state = match *char_next {
                            's' => DefinitionState::Inclusive,
                            'x' => DefinitionState::Exclusive,
                            _ => panic!("Not normal"),
                        };
                        let conditions_state = get_state(file)?;
                        for name in conditions_state {
                            if find_condition_state(&definition_list, &name) {
                                eprintln!(
                                    "{}:{}: start condition {} declared twice",
                                    file.name, file.line_nb, name
                                );
                            }
                            match state {
                                DefinitionState::Inclusive => {
                                    definition_list.push(Definition::InclusiveState {
                                        name: name,
                                        state_nb,
                                    })
                                }
                                DefinitionState::Exclusive => {
                                    definition_list.push(Definition::ExclusiveState {
                                        name: name,
                                        state_nb,
                                    })
                                }
                            }
                        }
                        state_nb += 1;
                    }
                }
            }
            '\n' => file.line_nb += 1,
            c => {
                if WHITESPACE.contains(c) {
                    let content = get_line_with_space(file);
                    definition_list.push(Definition::LineWithSpace { content });
                } else {
                    let definition_token = get_definition(c, file)?;
                    definition_list.push(definition_token);
                }
            }
        }
    }
    return Err("premature EOF".to_string());
}

fn find_condition_state(definitions: &[Definition], to_find: &String) -> bool {
    for elem in definitions {
        match elem {
            Definition::ExclusiveState { name, state_nb: _ }
            | Definition::InclusiveState { name, state_nb: _ } => {
                if name == to_find {
                    return true;
                }
            }
            _ => continue,
        }
    }
    return false;
}

pub fn get_all_condition_state(definitions: &[Definition]) -> Vec<(&String, DefinitionState)> {
    let mut dest = Vec::new();

    for elem in definitions {
        match elem {
            Definition::InclusiveState { name, state_nb: _ } => {
                dest.push((name, DefinitionState::Inclusive));
            }
            Definition::ExclusiveState { name, state_nb: _ } => {
                dest.push((name, DefinitionState::Exclusive));
            }
            _ => {}
        }
    }
    return dest;
}

pub fn is_inclusive_or_exclusive_state(
    definitions: &[Definition],
    state_name: &String,
) -> Result<DefinitionState, String> {
    let all_condition_state = get_all_condition_state(definitions);

    if state_name == "INITIAL" {
        return Ok(DefinitionState::Inclusive);
    }

    if let Some((_, state_type)) = all_condition_state.iter().find(|x| x.0 == state_name) {
        return Ok(*state_type);
    }
    return Err(format!("undeclared start condition {}", state_name));
}
