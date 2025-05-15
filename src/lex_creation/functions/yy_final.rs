use std::{fs::File, io::Write};

use crate::{file_parsing::FilePart, lex_creation::SPACE};

pub fn yy_final(file_parts: &FilePart, file: &mut File) -> std::io::Result<()> {
    let final_state = file_parts.actions();
    let hash = file_parts.action_hash();

    for (state, actions) in final_state {
        writeln!(file, "void final{}(int len_match) {{", state)?;

        for elem in actions.iter().rev() {
            let action = hash.get(elem).unwrap();

            writeln!(
                file,
                "{}yy_push_accepting_state({}, len_match);",
                SPACE, action
            )?;
        }
        writeln!(file, "}}")?;
    }
    write!(file, "\n")?;

    Ok(())
}
