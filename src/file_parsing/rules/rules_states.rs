use std::collections::HashSet;

use crate::file_parsing::{
    definitions::{
        definitions::{get_exclusive_state, get_inclusive_state},
        ConditionState, Definition, DefinitionState,
    },
    FileInfo,
};

fn extract_state_from_line(file: &mut FileInfo) -> Result<String, String> {
    let mut states_from_line = String::new();

    while let Some(char) = file.it.next() {
        match char {
            '\n' => {
                file.line_nb += 1;
                break;
            }
            'a'..'z' | 'A'..'Z' | '_' | ',' | '0'..'9' => {
                states_from_line.push(char);
            }
            '>' => {
                if states_from_line.len() > 1 && states_from_line.find('*').is_some() {
                    return Err("bad start condition list".to_string());
                }
                if states_from_line.is_empty() {
                    return Err("bad start condition list".to_string());
                }
                return Ok(states_from_line);
            }
            _ => {
                return Err(format!("bad <start condition>: {}", char));
            }
        }
    }
    return Err(format!("bad <start condition>"));
}

fn split_state_form_line(states: &String) -> Result<Vec<String>, String> {
    let mut all_states = Vec::new();
    let mut current_state_name = String::new();

    for char in states.chars() {
        match char {
            '0'..'9' => {
                if current_state_name.is_empty() {
                    return Err("bad <start condition>".to_string());
                }
                current_state_name.push(char);
            }
            ',' => {
                if current_state_name.is_empty() {
                    return Err("bad start condition list".to_string());
                }
                all_states.push(current_state_name.clone());
                current_state_name.clear();
            }
            _ => {
                current_state_name.push(char);
            }
        }
    }
    if current_state_name.is_empty() {
        return Err("bad start condition list".to_string());
    }
    all_states.push(current_state_name.clone());
    return Ok(all_states);
}

fn expand_star_for_state(definitions: &[Definition]) -> Vec<ConditionState> {
    let exclusive_states = get_exclusive_state(definitions);
    let inclusive_states = get_inclusive_state(definitions);

    let mut all_states = Vec::new();

    all_states.push(ConditionState::new(
        "INITIAL".to_string(),
        DefinitionState::Exclusive,
    ));

    if exclusive_states.is_some() {
        for state in exclusive_states.unwrap() {
            let new_def_state = ConditionState::new(state.clone(), DefinitionState::Exclusive);
            all_states.push(new_def_state);
        }
    }

    if inclusive_states.is_some() {
        for state in inclusive_states.unwrap() {
            let new_def_state = ConditionState::new(state.clone(), DefinitionState::Inclusive);
            all_states.push(new_def_state);
        }
    }
    return all_states;
}

fn is_inclusive_or_exclusive_state(
    state_name: &String,
    definitions: &[Definition],
) -> Result<DefinitionState, String> {
    let exclusive_states = get_exclusive_state(definitions);
    let inclusive_states = get_inclusive_state(definitions);

    if state_name == "INITIAL" {
        return Ok(DefinitionState::Inclusive);
    }

    if exclusive_states.is_some() {
        if let Some(_) = exclusive_states.unwrap().iter().find(|&x| x == state_name) {
            return Ok(DefinitionState::Exclusive);
        }
    }
    if inclusive_states.is_some() {
        if let Some(_) = inclusive_states.unwrap().iter().find(|&x| x == state_name) {
            return Ok(DefinitionState::Inclusive);
        }
    }

    return Err(format!("undeclared start condition {}", state_name));
}

fn warning_duplicate_condition_state_for_line(file: &mut FileInfo, state_list: &[ConditionState]) {
    let mut set = HashSet::with_capacity(state_list.len());

    for state in state_list {
        if set.contains(state) {
            eprintln!(
                "{}:{}: warning, <{}> specified twice",
                file.name,
                file.line_nb,
                state.name()
            );
        } else {
            set.insert(state);
        }
    }
}

fn find_states(
    all_states: &[String],
    definitions: &[Definition],
) -> Result<Vec<ConditionState>, String> {
    let mut state_list: Vec<ConditionState> = Vec::new();

    for state_name in all_states {
        match state_name.as_str() {
            "*" => {
                let mut star_states = expand_star_for_state(definitions);

                state_list.append(&mut star_states);
            }
            _ => {
                let state_type = is_inclusive_or_exclusive_state(state_name, definitions)?;
                let new_def_state = ConditionState::new(state_name.clone(), state_type);
                state_list.push(new_def_state);
            }
        }
    }
    return Ok(state_list);
}

pub fn extract_state_for_rule(
    file: &mut FileInfo,
    definitions: &[Definition],
) -> Result<Vec<ConditionState>, String> {
    let states = extract_state_from_line(file)?;

    let split_states = split_state_form_line(&states)?;

    dbg!(&split_states);

    let all_states_for_rule = find_states(&split_states, definitions)?;

    dbg!(&all_states_for_rule);

    warning_duplicate_condition_state_for_line(file, &all_states_for_rule);

    return Ok(all_states_for_rule);
}
