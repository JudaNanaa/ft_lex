use crate::file_parsing::{
    definitions::{ConditionState, Definition},
    rules::rules::process_rule_and_action,
    FileInfo,
};

fn skip_until_find(file: &mut FileInfo, to_find: char) -> Result<String, String> {
    let mut dest: String = String::new();

    while let Some(char) = file.it.next() {
        match char {
            '\\' => {
                dest.push('\\');
                if let Some(c) = file.it.next() {
                    dest.push(c);
                } else {
                    return Err(String::from("ERROR: end of file in string"));
                }
            }
            _ => {
                dest.push(char);
                if char == to_find {
                    return Ok(dest);
                }
            }
        }
    }
    return Err(String::from("ERROR: end of file in string"));
}

fn extract_state_block(file: &mut FileInfo) -> Result<String, String> {
    let mut state_block = String::new();
    while let Some(char) = file.it.next() {
        match char {
            '"' | '\'' => {
                state_block.push(char);
                state_block += skip_until_find(file, char)?.as_str();
            }
            _ => {
                if char == '}' {
                    return Ok(state_block);
                }
                state_block.push(char);
            }
        }
    }
    return Err(String::from("ERROR: end of file in string"));
}

pub fn parse_condition_state(
    file: &mut FileInfo,
    next_state_id: &mut usize,
    definitions: &[Definition],
) -> Result<(), String> {
    if let Some(char) = file.it.peek() {
        match char {
            '{' => {
                file.it.next();
                let in_state_block = extract_state_block(file)?;
                dbg!(&in_state_block);
            }
            ' ' | '\r' | '\t' | '\n' => {
                return Err("unrecognized rule".to_string());
            }
            _ => {
                let (nfa, action) = process_rule_and_action(file, next_state_id, definitions)?;

                dbg!(&nfa);
                dbg!(&action);
            }
        }
    } else {
        return Err("unrecognized rule".to_string());
    }
    return Ok(());
    // todo!();
}
