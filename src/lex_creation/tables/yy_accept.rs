use std::{fs::File, io::Write};

use crate::{lex_creation::SPACE, regex::dfa::DFA};

fn generate_accept_tab(dfa: &DFA) -> Vec<u8> {
    let nb_state = dfa.new_transitions().len();

    let mut tab = vec![0; nb_state];

    for state in dfa.new_transitions().keys() {
        if dfa.new_final_states().contains(state) {
            tab[*state] = 1;
        }
    }

    return tab;
}

pub fn yy_accept(dfa: &DFA, file: &mut File) -> std::io::Result<Vec<u8>> {
    let nb_state = dfa.new_transitions().len();

    let accept_tab = generate_accept_tab(dfa);

    writeln!(file, "\nconst int yy_accept[{}] =", nb_state)?;
    writeln!(file, "{}{{", SPACE)?;
    write!(file, "{}", SPACE.repeat(2))?;

    for (index, value) in accept_tab.iter().enumerate() {
        write!(file, "{}", value)?;
        if index != 0 && index % 10 == 0 {
            write!(file, ",")?;
            if index != accept_tab.len() - 1 {
                write!(file, "\n{}", SPACE.repeat(2))?;
            }
        } else if index != accept_tab.len() - 1 {
            write!(file, ",{}", SPACE)?;
        } else {
            writeln!(file, "")?;
        }
    }
    writeln!(file, "{}}} ;\n", SPACE)?;

    return Ok(accept_tab);
}
