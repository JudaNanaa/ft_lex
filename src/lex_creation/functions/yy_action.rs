use std::{collections::HashMap, fs::File, io::Write};

use crate::{
    file_parsing::{definitions::ConditionState, FilePart},
    lex_creation::SPACE,
};

fn get_key_based_on_value(hash: &HashMap<String, usize>, to_find: usize) -> Option<&String> {
    for (str, nb) in hash {
        if *nb == to_find {
            return Some(str);
        }
    }
    return None;
}

fn get_condition_state_for_action(file_parts: &FilePart, action: &str) -> Vec<ConditionState> {
    let rules_action = file_parts.rule_action();

    for elem in rules_action {
        if elem.action() == action {
            return elem.condition_state.clone();
        }
    }
    return vec![];
}

fn write_inner_condition_state_if(file: &mut File) -> std::io::Result<()> {
    writeln!(file, "{}{{", SPACE.repeat(2))?;
    writeln!(file, "{}REJECT;", SPACE.repeat(3))?;
    writeln!(file, "{}return;", SPACE.repeat(3))?;
    writeln!(file, "{}}}", SPACE.repeat(2))?;
    return Ok(());
}

fn is_only_initial(file: &mut File, condition_state: &[ConditionState]) -> std::io::Result<bool> {
    if condition_state.len() == 1 && condition_state[0].name() == "INITIAL" {
        writeln!(
            file,
            "{}if (yy_is_exclusive_state(state) == 1)",
            SPACE.repeat(2)
        )?;
        write_inner_condition_state_if(file)?;
        return Ok(true);
    }
    return Ok(false);
}

fn write_condition_state(
    file: &mut File,
    mut condition_state: Vec<ConditionState>,
) -> std::io::Result<()> {
    if is_only_initial(file, &condition_state)? == true {
        return Ok(());
    }

    write!(file, "{}if (", SPACE.repeat(2))?;
    while let Some(state) = condition_state.pop() {
        write!(file, "state != {}", state.name())?;
        if condition_state.len() != 0 {
            write!(file, " && ")?;
        }
    }
    writeln!(file, ")")?;
    write_inner_condition_state_if(file)?;
    return Ok(());
}

pub fn yy_action(file_parts: &FilePart, file: &mut File) -> std::io::Result<()> {
    let action_hash = file_parts.action_hash();

    writeln!(file, "void yy_action(int action) {{")?;
    writeln!(file, "{}switch (action) {{", SPACE)?;

    for nb_action in 1..=action_hash.len() {
        writeln!(file, "{}case {}:", SPACE.repeat(2), nb_action)?;
        let action = get_key_based_on_value(action_hash, nb_action).unwrap();
        let condition_state = get_condition_state_for_action(file_parts, &action);
        write_condition_state(file, condition_state)?;
        writeln!(file, "{}", action)?;
        writeln!(file, "{}break;", SPACE.repeat(2))?;
    }
    writeln!(file, "{}default:", SPACE.repeat(2))?;
    writeln!(file, "{}yy_fatal_error(\"not normal\");", SPACE.repeat(3))?;

    writeln!(file, "{}}}", SPACE)?;

    writeln!(file, "}}\n")?;

    Ok(())
}
