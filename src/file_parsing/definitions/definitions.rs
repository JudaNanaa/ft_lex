use crate::file_parsing::{
    definitions::{Definition, DefinitionState},
    FileInfo,
};

const WHITESPACE: &str = " \r\t";

fn extract_braced_content(file: &mut FileInfo) -> Result<String, String> {
    let mut content = String::new();

    while let Some(ch) = file.it.next() {
        if ch == '%' {
            if file.it.peek() == Some(&'}') {
                file.it.next();
                return Ok(content);
            }
        } else {
            if ch == '\n' {
                file.line_nb += 1;
            }
            content.push(ch);
        }
    }
    return Err("premature EOF".to_string());
}

fn read_spaced_line(file: &mut FileInfo) -> String {
    let mut content = String::new();

    while let Some(ch) = file.it.next() {
        if ch == '\n' {
            file.line_nb += 1;
            break;
        }
        content.push(ch);
    }
    return content;
}

fn parse_definition(file: &mut FileInfo) -> Result<Definition, String> {
    let mut line = String::new();

    while let Some(ch) = file.it.next() {
        if ch == '\n' {
            file.line_nb += 1;
            break;
        }
        line.push(ch);
    }

    let trimmed = line.trim().to_string();
    if let Some(idx) = trimmed.find(' ') {
        let name = trimmed[0..idx].to_string();
        let value = trimmed[idx + 1..].trim().to_string();
        return Ok(Definition::Definition { name, value });
    } else {
        return Err("incomplete name definition".to_string());
    }
}

fn split_by_chars(s: &str, delimiters: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut chars = s.chars();

    while let Some(ch) = chars.next() {
        if delimiters.contains(ch) {
            if !current.is_empty() {
                result.push(current.clone());
                current.clear();
            }
        } else {
            current.push(ch);
        }
    }
    if !current.is_empty() {
        result.push(current);
    }
    return result;
}

fn parse_state_line(file: &mut FileInfo) -> Result<Vec<String>, String> {
    let mut content = String::new();

    file.it.next(); // Skip the 's' or 'x'
    for ch in file.it.by_ref() {
        if ch == '\n' {
            file.line_nb += 1;
            let split = split_by_chars(&content, " \t");
            if split.is_empty() {
                return Err("bad start condition list".to_string());
            }
            return Ok(split);
        }
        content.push(ch);
    }
    return Err("Premature EOF".to_string());
}

fn advance_to_newline(file: &mut FileInfo) {
    for ch in file.it.by_ref() {
        if ch == '\n' {
            file.line_nb += 1;
            return;
        }
    }
}

pub fn parse_definitions(file: &mut FileInfo) -> Result<Vec<Definition>, String> {
    let mut defs = Vec::new();
    let mut state_id = 1;

    while let Some(ch) = file.it.peek() {
        match ch {
            '%' => {
                file.it.next();
                if let Some(next) = file.it.peek() {
                    match next {
                        '{' => {
                            file.it.next();
                            let content = extract_braced_content(file)?;
                            defs.push(Definition::Bloc { content });
                        }
                        '%' => {
                            advance_to_newline(file);
                            return Ok(defs);
                        }
                        's' | 'x' => {
                            let state_type = if *next == 's' {
                                DefinitionState::Inclusive
                            } else {
                                DefinitionState::Exclusive
                            };
                            let states = parse_state_line(file)?;
                            for name in states {
                                if state_exists(&defs, &name) {
                                    eprintln!(
                                        "{}:{}: start condition {} declared twice",
                                        file.name, file.line_nb, name
                                    );
                                }
                                let def = if matches!(state_type, DefinitionState::Inclusive) {
                                    Definition::InclusiveState {
                                        name,
                                        state_nb: state_id,
                                    }
                                } else {
                                    Definition::ExclusiveState {
                                        name,
                                        state_nb: state_id,
                                    }
                                };
                                defs.push(def);
                            }
                            state_id += 1;
                        }
                        _ => {}
                    }
                }
            }
            '\n' => {
                file.it.next();
                file.line_nb += 1
            }
            c if WHITESPACE.contains(*c) => {
                let content = read_spaced_line(file);
                defs.push(Definition::LineWithSpace { content });
            }
            _ => {
                let def = parse_definition(file)?;
                defs.push(def);
            }
        }
    }
    return Err("premature EOF".to_string());
}

fn state_exists(defs: &[Definition], target: &str) -> bool {
    return defs.iter().any(|def| matches!(def, Definition::ExclusiveState { name, .. } | Definition::InclusiveState { name, .. } if name == target));
}

pub fn list_all_states(defs: &[Definition]) -> Vec<(&String, DefinitionState)> {
    let mut result = Vec::new();

    for def in defs {
        match def {
            Definition::InclusiveState { name, .. } => {
                result.push((name, DefinitionState::Inclusive))
            }
            Definition::ExclusiveState { name, .. } => {
                result.push((name, DefinitionState::Exclusive))
            }
            _ => {}
        }
    }
    return result;
}

pub fn get_state_type(defs: &[Definition], state_name: &str) -> Result<DefinitionState, String> {
    let states = list_all_states(defs);

    if state_name == "INITIAL" {
        return Ok(DefinitionState::Inclusive);
    }

    return states
        .iter()
        .find(|(name, _)| name == &state_name)
        .map(|&(_, typ)| Ok(typ))
        .unwrap_or_else(|| Err(format!("undeclared start condition {}", state_name)));
}
