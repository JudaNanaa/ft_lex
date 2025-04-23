use std::char;

use crate::regex::{nfa::nfa::construct_nfa, regex_tokenizer};

use super::{Definition, FileInfo, RuleAction};

fn add_to_next_quote(rule: &mut String, file: &mut FileInfo) -> Result<(), &'static str> {
    rule.push('"');
    while let Some(char) = file.it.next() {
        match char {
            '\n' => {
                file.line_nb += 1;
                return Err("missing quote");
            }
            '"' => {
                break;
            }
            _ => {
                rule.push(char);
            }
        }
    }
    return Ok(());
}

fn find_definition(
    name_def: String,
    definitions: &Vec<Definition>,
) -> Result<String, &'static str> {
    match name_def.chars().nth(0) {
        None => {
            todo!();
        }
        Some(char) => {
            if char.is_digit(10) {
                return Ok("{".to_owned() + &name_def + "}");
            }
        }
    }
    for def in definitions {
        match def {
            Definition::Definition { name, value } => {
                if &name_def == name {
                    return Ok("(".to_owned() + &value.clone() + ")");
                }
            }
            _ => {}
        }
    }
    return Err("Not found in def");
}

fn extract_brace(
    rule: &mut String,
    file: &mut FileInfo,
    definitions: &Vec<Definition>,
) -> Result<(), &'static str> {
    let mut name = String::new();

    while let Some(char) = file.it.next() {
        match char {
            '\n' => {
                file.line_nb += 1;
                return Err("missing }");
            }
            '}' => {
                // TODO name empty ?
                let def_value = find_definition(name, definitions)?;
                rule.push_str(&def_value);
                return Ok(());
            }
            _ => {
                name.push(char);
            }
        }
    }
    return Err("unrecognized rule");
}

fn get_content_under_quotes_action(
    file: &mut FileInfo,
    quote_to_match: char,
) -> Result<String, &'static str> {
    let mut result = String::new();

    result.push(quote_to_match);
    while let Some(char) = file.it.next() {
        match char {
            '\\' => {
                result.push('\\');
                if let Some(c) = file.it.next() {
                    result.push(c);
                }
            }
            '\n' => {
                return Err("missing quote");
            }
            c if c == quote_to_match => {
                result.push(quote_to_match);
                return Ok(result);
            }
            _ => {
                result.push(char);
            }
        }
    }

    todo!();
}

fn get_content_under_brace_action(file: &mut FileInfo) -> Result<String, &'static str> {
    let mut result = String::new();

    result.push('{');
    while let Some(char) = file.it.next() {
        match char {
            '{' => {
                let test = get_content_under_brace_action(file)?;
                result.push_str(&test);
            }
            '}' => {
                result.push('}');
                return Ok(result);
            }
            '\'' | '"' => {
                let test = get_content_under_quotes_action(file, char)?;
                result.push_str(&test);
            }
            _ => result.push(char),
        }
    }
    return Err("unclose brace action");
}

fn extract_action(file: &mut FileInfo) -> Result<String, &'static str> {
    let mut action = String::new();

    while let Some(char) = file.it.next() {
        match char {
            '\n' => {
                return Ok(action);
            }
            '{' => {
                let test = get_content_under_brace_action(file)?;
                action.push_str(&test);
            }
            '}' => {
                todo!("error unclosed")
            }
            '\'' | '"' => {
                let test = get_content_under_quotes_action(file, char)?;
                action.push_str(&test);
            }
            _ => action.push(char),
        }
    }
    return Err("EOF encouter action");
}

fn split_rule_action(
    file: &mut FileInfo,
    first_char: char,
    definitions: &Vec<Definition>,
) -> Result<(String, String), &'static str> {
    let mut rule = String::new();

    match first_char {
        '"' => {
            add_to_next_quote(&mut rule, file)?;
        }
        '{' => {
            extract_brace(&mut rule, file, definitions)?;
        }
        '}' => {
            return Err("unrecognized rule");
        }
        _ => rule.push(first_char),
    }

    while let Some(char) = file.it.next() {
        match char {
            '"' => {
                add_to_next_quote(&mut rule, file)?;
            }
            ' ' | '\t' => {
                break;
            }
            '{' => {
                extract_brace(&mut rule, file, definitions)?;
            }
            '}' => {
                return Err("unrecognized rule");
            }
            _ => {
                rule.push(char);
            }
        }
    }

    let action = extract_action(file)?.trim().to_string();
    dbg!(&rule);
    dbg!(&action);
    return Ok((rule, action));
}

pub fn parse_rules_part(
    file: &mut FileInfo,
    definitions: &Vec<Definition>,
) -> Result<(Vec<RuleAction>, Vec<String>), String> {
    let mut texts: Vec<String> = Vec::new();
    let mut rules: Vec<RuleAction> = Vec::new();
    let mut start_state = 1;

    while let Some(char) = file.it.next() {
        match char {
            '\n' => {
                file.line_nb += 1;
                continue;
            }
            '%' => {
                if let Some(next_char) = file.it.peek() {
                    if *next_char == '%' {
                        file.it.next();
                        return Ok((rules, texts));
                    }
                }
            }
            ' ' | '\t' => {
                let mut text = String::new();
                while let Some(char) = file.it.next() {
                    match char {
                        '\n' => {
                            file.line_nb += 1;
                            break;
                        }
                        _ => text.push(char),
                    }
                }
                dbg!(&text);
                texts.push(text);
            }
            c => {
                let (rule, action) = split_rule_action(file, c, &definitions)?;
                let tokens = regex_tokenizer(&rule);
                let (nfa, new_id) = construct_nfa(&tokens, start_state);
                start_state = new_id;
                rules.push(RuleAction { nfa, action });
            }
        }
    }

    return Ok((rules, texts));
}
