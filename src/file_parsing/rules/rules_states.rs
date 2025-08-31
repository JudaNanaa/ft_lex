use std::collections::HashSet;

use crate::file_parsing::{
    definitions::{
        definitions::{get_all_condition_state, is_inclusive_or_exclusive_state},
        ConditionState, Definition,
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
            'a'..'z' | 'A'..'Z' | '_' | ',' | '0'..'9' | '*' => {
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
    let all_condition_states = get_all_condition_state(definitions);

    let mut all_states = Vec::new();

    all_states.push(ConditionState::initial());

    for (name, state_type) in all_condition_states {
        all_states.push(ConditionState::new(name.clone(), state_type));
    }
    return all_states;
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
                let state_type = is_inclusive_or_exclusive_state(definitions, state_name)?;
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

    let all_states_for_rule = find_states(&split_states, definitions)?;

    warning_duplicate_condition_state_for_line(file, &all_states_for_rule);

    return Ok(all_states_for_rule);
}
