use crate::file_parsing::{
    definitions::{ConditionState, Definition},
    rules::rules::process_rule_and_action,
    FileInfo,
};

fn skip_until_find(file: &mut FileInfo, to_find: char) -> Option<()> {
    while let Some(char) = file.it.next() {
        if char == to_find {
            return Some(());
        }
    }
    return None;
}

fn extract_state_block(file: &mut FileInfo) -> Result<String, String> {
    while let Some(char) = file.it.next() {
        match char {
            '"' | '\'' => {
                if skip_until_find(file, char).is_none() {
                    return Err("ERROR: end of file in string".to_string());
                }
            }
            // TODO continuer ici
            _ => todo!(),
        }
    }
    todo!();
}

pub fn parse_condition_state(
    file: &mut FileInfo,
    next_state_id: &mut usize,
    definitions: &[Definition],
) -> Result<(), String> {
    if let Some(char) = file.it.next() {
        match char {
            '{' => {}
            ' ' | '\r' | '\t' | '\n' => {
                return Err("unrecognized rule".to_string());
            }
            _ => {
                let (nfa, action) = process_rule_and_action(file, next_state_id, definitions)?;
            }
        }
    } else {
        return Err("unrecognized rule".to_string());
    }
    todo!();
}
