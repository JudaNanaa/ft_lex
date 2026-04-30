use std::{fs::File, io::Write};

use crate::{file_parsing::FilePart, lex_creation::SPACE};

pub fn yy_final(file_parts: &FilePart, file: &mut File) -> std::io::Result<()> {
    let final_state = file_parts.actions();
    let hash = file_parts.map_actions();

    for (state, actions) in final_state {
        writeln!(file, "void final{state}(int len_match) {{")?;

        for elem in actions.iter().rev() {
            let action = hash.get(elem).unwrap();

            writeln!(
                file,
                "{SPACE}yy_push_accepting_state({action}, {state}, len_match);",
            )?;
        }
        writeln!(file, "}}")?;
    }
    writeln!(file)?;

    Ok(())
}
