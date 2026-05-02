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
    Err("premature EOF".to_string())
}

fn read_spaced_line(file: &mut FileInfo) -> String {
    let mut content = String::new();

    for ch in file.it.by_ref() {
        if ch == '\n' {
            file.line_nb += 1;
            break;
        }
        content.push(ch);
    }
    content
}

fn parse_definition(file: &mut FileInfo) -> Result<Definition, String> {
    let mut line = String::new();

    for ch in file.it.by_ref() {
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
        Ok(Definition::MacroDef { name, value })
    } else {
        Err("incomplete name definition".to_string())
    }
}

fn parse_state_line(file: &mut FileInfo) -> Result<Vec<String>, String> {
    let mut content = String::new();

    file.it.next(); // Skip the 's' or 'x'
    for ch in file.it.by_ref() {
        if ch == '\n' {
            file.line_nb += 1;
            let split: Vec<String> = content
                .split([' ', '\t'])
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect();
            if split.is_empty() {
                return Err("bad start condition list".to_string());
            }
            return Ok(split);
        }
        content.push(ch);
    }
    Err("Premature EOF".to_string())
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
                        'o' => {
                            let rest = read_spaced_line(file);
                            if let Some(opts) = rest.trim().strip_prefix("ption") {
                                for token in opts.split_whitespace() {
                                    defs.push(Definition::Option {
                                        name: token.to_string(),
                                    });
                                }
                            }
                        }
                        'a' | 'p' => {
                            let rest = read_spaced_line(file);
                            let keyword = rest.trim();
                            if keyword == "array" || keyword == "pointer" {
                                defs.push(Definition::Option {
                                    name: keyword.to_string(),
                                });
                            }
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
                file.line_nb += 1;
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
    Err("premature EOF".to_string())
}

fn state_exists(defs: &[Definition], target: &str) -> bool {
    defs.iter().any(|def| matches!(def, Definition::ExclusiveState { name, .. } | Definition::InclusiveState { name, .. } if name == target))
}

pub fn list_all_states(defs: &[Definition]) -> Vec<(&String, DefinitionState)> {
    let mut result = Vec::new();

    for def in defs {
        match def {
            Definition::InclusiveState { name, .. } => {
                result.push((name, DefinitionState::Inclusive));
            }
            Definition::ExclusiveState { name, .. } => {
                result.push((name, DefinitionState::Exclusive));
            }
            _ => {}
        }
    }
    result
}

pub fn get_state_type(defs: &[Definition], state_name: &str) -> Result<DefinitionState, String> {
    let states = list_all_states(defs);

    if state_name == "INITIAL" {
        return Ok(DefinitionState::Inclusive);
    }

    states
        .iter()
        .find(|(name, _)| name == &state_name)
        .map_or_else(
            || Err(format!("undeclared start condition {state_name}")),
            |&(_, typ)| Ok(typ),
        )
}
