use std::{char, collections::HashMap};

use crate::{
    file_parsing::{
        definitions::{ConditionState, Definition},
        rules::{
            condition_state::parse_condition_states, rules_states::extract_rule_states, RuleAction,
        },
        FileInfo,
    },
    regex::{nfa::nfa::build_nfa, regex_tokenizer, NFA},
};

pub fn map_actions(rules: &[RuleAction]) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    let mut index = 1;

    for rule in rules {
        if rule.action != "|" && !map.contains_key(&rule.action) {
            map.insert(rule.action.clone(), index);
            index += 1;
        }
    }
    return map;
}

fn append_quoted(rule: &mut String, file: &mut FileInfo) -> Result<(), String> {
    rule.push('"');
    while let Some(ch) = file.it.next() {
        match ch {
            '\n' => {
                file.line_nb += 1;
                return Err("missing quote".to_string());
            }
            '\\' => {
                if let Some(esc) = file.it.next() {
                    if esc == '\n' {
                        file.line_nb += 1;
                        return Err("missing quote".to_string());
                    }
                    rule.push_str(&['\\', esc].iter().collect::<String>());
                } else {
                    return Err("missing quote".to_string());
                }
            }
            '"' => {
                rule.push('"');
                return Ok(());
            }
            _ => rule.push(ch),
        }
    }
    return Err("missing quote".to_string());
}

fn resolve_def(name: &str, defs: &[Definition]) -> Result<String, String> {
    if name.starts_with(|c: char| c.is_ascii_digit()) {
        return Ok(format!("{{{}}}", name));
    }
    for def in defs.iter() {
        if let Definition::Definition { name: n, value } = def {
            if n == name {
                return Ok(format!("({})", value));
            }
        }
    }
    return Err("Definition not found".to_string());
}

fn extract_def(rule: &mut String, file: &mut FileInfo, defs: &[Definition]) -> Result<(), String> {
    let mut name = String::new();
    for ch in file.it.by_ref() {
        match ch {
            '\n' => {
                file.line_nb += 1;
                return Err("missing }".to_string());
            }
            '}' => {
                rule.push_str(&resolve_def(&name, defs)?);
                return Ok(());
            }
            _ => name.push(ch),
        }
    }
    return Err("unterminated brace block".to_string());
}

fn extract_char_class(rule: &mut String, file: &mut FileInfo) -> Result<(), String> {
    rule.push('[');
    let mut posix_buf = String::new();
    let mut in_posix = false;

    while let Some(ch) = file.it.next() {
        match ch {
            '\n' => {
                file.line_nb += 1;
                return Err("missing ]".to_string());
            }
            '[' => {
                posix_buf.push('[');
                in_posix = true;
            }
            ']' if in_posix => {
                posix_buf.push(']');
                let expansion = match posix_buf.as_str() {
                    "[:alnum:]" => "A-Za-z0-9",
                    "[:alpha:]" => "A-Za-z",
                    "[:digit:]" => "0-9",
                    "[:lower:]" => "a-z",
                    "[:upper:]" => "A-Z",
                    "[:xdigit:]" => "A-Fa-f0-9",
                    "[:space:]" => r" \t\r\n\v\f",
                    "[:blank:]" => " \t",
                    "[:cntrl:]" => "\x00-\x1F\x7F",
                    "[:punct:]" => "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
                    "[:print:]" => " -~",
                    "[:graph:]" => "!-~",
                    _ => {
                        if posix_buf.contains("[:") && posix_buf.contains(":]") {
                            return Err("unknown POSIX class".to_string());
                        }
                        posix_buf.as_str()
                    }
                };
                rule.push_str(expansion);
                in_posix = false;
            }
            ']' => {
                rule.push(']');
                return Ok(());
            }
            _ if in_posix => posix_buf.push(ch),
            _ => rule.push(ch),
        }
    }
    return Err("unterminated character class".to_string());
}

fn read_quoted_action(file: &mut FileInfo, quote: char) -> Result<String, String> {
    let mut result = String::from(quote);
    while let Some(ch) = file.it.next() {
        match ch {
            '\\' => {
                result.push('\\');
                if let Some(esc) = file.it.next() {
                    result.push(esc);
                }
            }
            '\n' => {
                file.line_nb += 1;
                return Err("missing quote".to_string());
            }
            c if c == quote => {
                result.push(quote);
                return Ok(result);
            }
            _ => result.push(ch),
        }
    }
    return Err("unterminated quoted action".to_string());
}

fn read_braced_action(file: &mut FileInfo) -> Result<String, String> {
    let mut result = String::from('{');
    while let Some(ch) = file.it.next() {
        match ch {
            '{' => result.push_str(&read_braced_action(file)?),
            '}' => {
                result.push('}');
                return Ok(result);
            }
            '\'' | '"' => result.push_str(&read_quoted_action(file, ch)?),
            _ => result.push(ch),
        }
    }
    return Err("unclosed brace in action".to_string());
}

fn parse_action(file: &mut FileInfo) -> Result<String, String> {
    let mut action = String::new();
    while let Some(ch) = file.it.next() {
        match ch {
            '\n' => {
                file.line_nb += 1;
                return Ok(action);
            }
            '{' => action.push_str(&read_braced_action(file)?),
            '}' => return Err("unbalanced closing brace".to_string()),
            '\'' | '"' => action.push_str(&read_quoted_action(file, ch)?),
            _ => action.push(ch),
        }
    }
    return Err("unexpected EOF while reading action".to_string());
}

fn parse_rule_action(file: &mut FileInfo, defs: &[Definition]) -> Result<(String, String), String> {
    let mut rule = String::new();
    while let Some(ch) = file.it.next() {
        match ch {
            '"' => append_quoted(&mut rule, file)?,
            ' ' | '\t' => break,
            '{' => extract_def(&mut rule, file, defs)?,
            '[' => extract_char_class(&mut rule, file)?,
            '}' | ']' => return Err("unexpected closing character".to_string()),
            '\\' => {
                rule.push('\\');
                if let Some(c) = file.it.next() {
                    rule.push(c);
                } else {
                    return Err("unrecognized rule".to_string());
                }
            }
            _ => rule.push(ch),
        }
    }
    let action = parse_action(file)?.trim().to_string();
    return Ok((rule, action));
}

pub fn build_rule_nfa(
    file: &mut FileInfo,
    next_state_id: &mut usize,
    defs: &[Definition],
) -> Result<(NFA, String), String> {
    let (rule, action) = parse_rule_action(file, defs)?;
    let tokens = regex_tokenizer(&rule);
    let nfa = build_nfa(&tokens, next_state_id);
    return Ok((nfa, action));
}

pub fn parse_rules(
    file: &mut FileInfo,
    defs: &[Definition],
) -> Result<(Vec<RuleAction>, Vec<String>), String> {
    let mut yylex_lines = Vec::new();
    let mut rules = Vec::new();
    let mut next_state_id = 1;

    while let Some(ch) = file.it.peek() {
        match ch {
            '\n' => {
                file.line_nb += 1;
                file.it.next();
            }
            '%' => {
                file.it.next();
                if file.it.peek() == Some(&'%') {
                    file.it.next();
                    return Ok((rules, yylex_lines));
                }
            }
            ' ' | '\t' => {
                file.it.next();
                let mut line = String::new();
                for ch in file.it.by_ref() {
                    if ch == '\n' {
                        file.line_nb += 1;
                        break;
                    }
                    line.push(ch);
                }
                yylex_lines.push(line);
            }
            '<' => {
                file.it.next();
                let states = extract_rule_states(file, defs)?;
                let mut state_rules =
                    parse_condition_states(file, &mut next_state_id, defs, &states)?;
                rules.append(&mut state_rules);
            }
            _ => {
                let (nfa, action) = build_rule_nfa(file, &mut next_state_id, defs)?;
                rules.push(RuleAction {
                    nfa,
                    action,
                    condition_state: vec![ConditionState::initial()],
                });
            }
        }
    }
    return Ok((rules, yylex_lines));
}
