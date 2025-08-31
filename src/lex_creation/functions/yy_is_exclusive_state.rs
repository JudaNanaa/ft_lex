use std::{fs::File, io::Write};

use crate::{
    file_parsing::{
        definitions::{definitions::get_all_condition_state, DefinitionState},
        FilePart,
    },
    lex_creation::SPACE,
};

pub fn write_yy_is_exclusive_state(file_parts: &FilePart, file: &mut File) -> std::io::Result<()> {
    let definitions = file_parts.definitions();

    let all_condition_state = get_all_condition_state(definitions);

    writeln!(file, "int yy_is_exclusive_state(int state) {{")?;
    writeln!(file, "{}switch (state) {{", SPACE)?;

    for (state_name, state_type) in all_condition_state {
        if state_type == DefinitionState::Exclusive {
            writeln!(file, "{}case {}:", SPACE.repeat(2), state_name)?;
            writeln!(file, "{}return 1;", SPACE.repeat(3))?;
        }
    }
    writeln!(file, "{}default:", SPACE.repeat(2))?;
    writeln!(file, "{}return 0;", SPACE.repeat(3))?;
    writeln!(file, "{}}}", SPACE)?;
    writeln!(file, "}}\n")?;
    return Ok(());
}
