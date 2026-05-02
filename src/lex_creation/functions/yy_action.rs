use std::collections::HashMap;

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
    None
}

fn get_condition_state_for_action(file_parts: &FilePart, action: &str) -> Vec<ConditionState> {
    for elem in file_parts.rule_action() {
        if elem.action() == action {
            return elem.condition_state.clone();
        }
    }
    vec![]
}

fn get_bol_for_action(file_parts: &FilePart, action: &str) -> bool {
    for elem in file_parts.rule_action() {
        if elem.action() == action {
            return elem.anchored_start;
        }
    }
    false
}

fn get_eol_for_action(file_parts: &FilePart, action: &str) -> bool {
    for elem in file_parts.rule_action() {
        if elem.action() == action {
            return elem.anchored_end;
        }
    }
    false
}

fn write_inner_condition_state_if(file: &mut dyn std::io::Write) -> std::io::Result<()> {
    writeln!(file, "{}{{", SPACE.repeat(2))?;
    writeln!(file, "{}REJECT;", SPACE.repeat(3))?;
    writeln!(file, "{}return;", SPACE.repeat(3))?;
    writeln!(file, "{}}}", SPACE.repeat(2))?;
    Ok(())
}

fn is_only_initial(
    file: &mut dyn std::io::Write,
    condition_state: &[ConditionState],
) -> std::io::Result<bool> {
    if condition_state.len() == 1 && condition_state[0].name() == "INITIAL" {
        writeln!(
            file,
            "{}if (yy_is_exclusive_state(state) == 1)",
            SPACE.repeat(2)
        )?;
        write_inner_condition_state_if(file)?;
        return Ok(true);
    }
    Ok(false)
}

fn write_condition_state(
    file: &mut dyn std::io::Write,
    mut condition_state: Vec<ConditionState>,
) -> std::io::Result<()> {
    if is_only_initial(file, &condition_state)? {
        return Ok(());
    }

    write!(file, "{}if (", SPACE.repeat(2))?;
    while let Some(state) = condition_state.pop() {
        write!(file, "state != {}", state.name())?;
        if !condition_state.is_empty() {
            write!(file, " && ")?;
        }
    }
    writeln!(file, ")")?;
    write_inner_condition_state_if(file)?;
    Ok(())
}

pub fn yy_action(file_parts: &FilePart, file: &mut dyn std::io::Write) -> std::io::Result<()> {
    let action_hash = file_parts.map_actions();

    let has_anchored_start = file_parts.rule_action().iter().any(|r| r.anchored_start);
    let has_anchored_end = file_parts.rule_action().iter().any(|r| r.anchored_end);

    if has_anchored_start {
        writeln!(file, "extern int yy_at_bol;")?;
    }
    if has_anchored_end {
        writeln!(file, "int yy_at_eol(void);")?;
    }

    writeln!(file, "void yy_action(int action) {{")?;
    writeln!(file, "{SPACE}switch (action) {{")?;

    for nb_action in 1..=action_hash.len() {
        writeln!(file, "{}case {}:", SPACE.repeat(2), nb_action)?;
        let action = get_key_based_on_value(action_hash, nb_action).unwrap();
        let condition_state = get_condition_state_for_action(file_parts, action);
        write_condition_state(file, condition_state)?;
        if get_bol_for_action(file_parts, action) {
            writeln!(
                file,
                "{}if (!yy_at_bol) {{ REJECT; return; }}",
                SPACE.repeat(2)
            )?;
        }
        if get_eol_for_action(file_parts, action) {
            writeln!(
                file,
                "{}if (!yy_at_eol()) {{ REJECT; return; }}",
                SPACE.repeat(2)
            )?;
        }
        writeln!(file, "{action}")?;
        writeln!(file, "{}break;", SPACE.repeat(2))?;
    }
    writeln!(file, "{}default:", SPACE.repeat(2))?;
    writeln!(file, "{}yy_fatal_error(\"not normal\");", SPACE.repeat(3))?;

    writeln!(file, "{SPACE}}}")?;

    writeln!(file, "}}\n")?;

    Ok(())
}
