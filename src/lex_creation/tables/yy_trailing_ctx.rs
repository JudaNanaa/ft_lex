use std::{fs::File, io::Write};

use crate::{lex_creation::SPACE, regex::dfa::DFA};

pub fn yy_trailing_ctx(dfa: &DFA, file: &mut File) -> std::io::Result<()> {
    if dfa.trailing_context_final_states.is_none() {
        return Ok(());
    }

    let trailing_ctx = dfa.trailing_context_final_states.as_ref().unwrap();

    let nb_trailing_context = trailing_ctx.iter().len();

    writeln!(
        file,
        "\nconst int yy_trailing_ctx[{}] =",
        nb_trailing_context
    )?;
    writeln!(file, "{}{{", SPACE)?;
    write!(file, "{}", SPACE.repeat(2))?;

    for (index, value) in trailing_ctx.iter().enumerate() {
        write!(file, "{}", value)?;
        if index != 0 && index % 10 == 0 {
            write!(file, ",")?;
            if index != nb_trailing_context - 1 {
                write!(file, "\n{}", SPACE.repeat(2))?;
            }
        } else if index != nb_trailing_context - 1 {
            write!(file, ",{}", SPACE)?;
        } else {
            writeln!(file)?;
        }
    }
    writeln!(file, "{}}} ;\n", SPACE)?;

    return Ok(());
}
