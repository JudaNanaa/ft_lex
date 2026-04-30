use std::{fs::File, io::Write};

use crate::{lex_creation::SPACE, regex::dfa::DFA};

pub fn yy_trailing(dfa: &DFA, file: &mut File) -> std::io::Result<()> {
    let nb_state = dfa.transitions().len();
    let mut tab = vec![0u8; nb_state];

    for &state in &dfa.trailing_states {
        if state < nb_state {
            tab[state] = 1;
        }
    }

    writeln!(file, "\nconst int yy_trailing[{}] =", nb_state)?;
    writeln!(file, "{}{{", SPACE)?;
    write!(file, "{}", SPACE.repeat(2))?;

    for (index, value) in tab.iter().enumerate() {
        write!(file, "{}", value)?;
        if index != 0 && index % 10 == 0 {
            write!(file, ",")?;
            if index != tab.len() - 1 {
                write!(file, "\n{}", SPACE.repeat(2))?;
            }
        } else if index != tab.len() - 1 {
            write!(file, ",{}", SPACE)?;
        } else {
            writeln!(file)?;
        }
    }
    writeln!(file, "{}}} ;\n", SPACE)?;

    Ok(())
}
