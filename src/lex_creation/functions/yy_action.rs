use std::{collections::HashMap, fs::File, io::Write};

use crate::{file_parsing::FilePart, lex_creation::SPACE};

fn get_key_based_on_value(hash: &HashMap<String, usize>, to_find: usize) -> Option<&String> {
    for (str, nb) in hash {
        if *nb == to_find {
            return Some(str);
        }
    }
    return None;
}

pub fn yy_action(file_parts: &FilePart, file: &mut File) -> std::io::Result<()> {
    let action_hash = file_parts.action_hash();

    writeln!(file, "void yy_action(int action) {{")?;
    writeln!(file, "{}switch (action) {{", SPACE)?;

    for nb_action in 1..=action_hash.len() {
        writeln!(file, "{}case {}:", SPACE.repeat(2), nb_action)?;
        let action = get_key_based_on_value(action_hash, nb_action).unwrap();
        writeln!(file, "{}", action)?;
        writeln!(file, "{}break;", SPACE.repeat(2))?;
    }
    writeln!(file, "{}default:", SPACE.repeat(2))?;
    writeln!(file, "{}yy_fatal_error(\"not normal\");", SPACE.repeat(3))?;

    writeln!(file, "{}}}", SPACE)?;

    writeln!(file, "}}\n")?;

    Ok(())
}
