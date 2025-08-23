use crate::file_parsing::{
    definitions::{ConditionState, Definition},
    rules::{rules::process_rule_and_action, RuleAction},
    FileInfo,
};

fn skip_until_newline_state_block(file: &mut FileInfo) -> Result<(), String> {
    while let Some(char) = file.it.peek() {
        match char {
            '\n' => {
                file.it.next();
                file.line_nb += 1;
                break;
            }
            ' ' | '\t' => {
                file.it.next();
                continue;
            }
            _ => return Err("unrecognized rule".to_string()),
        }
    }
    return Ok(());
}

fn extract_state_block(
    file: &mut FileInfo,
    next_state_id: &mut usize,
    definitions: &[Definition],
    state_list: &[ConditionState],
) -> Result<Vec<RuleAction>, String> {
    let mut rules_from_state_block = Vec::new();

    if let Some('}') = file.it.peek() {
        file.it.next();
        return Ok(rules_from_state_block);
    }

    skip_until_newline_state_block(file)?;

    while let Some(char) = file.it.peek() {
        match char {
            '\n' | ' ' | '\t' => {
                if *char == '\n' {
                    file.line_nb += 1;
                }
                file.it.next();
            }

            _ => {
                if *char == '}' {
                    file.it.next();
                    return Ok(rules_from_state_block);
                }

                let (nfa, action) = process_rule_and_action(file, next_state_id, definitions)?;
                rules_from_state_block.push(RuleAction {
                    nfa,
                    action,
                    condition_state: state_list.to_vec(),
                });
            }
        }
    }
    return Err(String::from("ERROR: end of file in string"));
}

pub fn parse_condition_state(
    file: &mut FileInfo,
    next_state_id: &mut usize,
    definitions: &[Definition],
    state_list: &[ConditionState],
) -> Result<Vec<RuleAction>, String> {
    if let Some(char) = file.it.peek() {
        match char {
            '{' => {
                file.it.next();
                let rules_from_state_block =
                    extract_state_block(file, next_state_id, definitions, state_list)?;
                dbg!(&rules_from_state_block);
                return Ok(rules_from_state_block);
            }
            ' ' | '\r' | '\t' | '\n' => {
                if *char == '\n' {
                    file.line_nb += 1;
                }
                return Err("unrecognized rule".to_string());
            }
            _ => {
                let (nfa, action) = process_rule_and_action(file, next_state_id, definitions)?;
                dbg!(&nfa);
                dbg!(&action);

                return Ok(vec![RuleAction {
                    nfa,
                    action,
                    condition_state: state_list.to_vec(),
                }]);
            }
        }
    }
    return Err("unrecognized rule".to_string());
}
