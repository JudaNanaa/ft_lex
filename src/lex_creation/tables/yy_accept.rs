use crate::{lex_creation::SPACE, regex::dfa::Dfa};

fn generate_accept_tab(dfa: &Dfa) -> Vec<u8> {
    let nb_state = dfa.transitions().len();

    let mut tab = vec![0; nb_state];

    for state in dfa.transitions().keys() {
        if dfa.final_states().contains(state) {
            tab[*state] = 1;
        }
    }

    tab
}

pub fn yy_accept(dfa: &Dfa, file: &mut dyn std::io::Write) -> std::io::Result<Vec<u8>> {
    let nb_state = dfa.transitions().len();

    let accept_tab = generate_accept_tab(dfa);

    writeln!(file, "\nconst int yy_accept[{nb_state}] =")?;
    writeln!(file, "{SPACE}{{")?;
    write!(file, "{}", SPACE.repeat(2))?;

    for (index, value) in accept_tab.iter().enumerate() {
        write!(file, "{value}")?;
        if index != 0 && index % 10 == 0 {
            write!(file, ",")?;
            if index != accept_tab.len() - 1 {
                write!(file, "\n{}", SPACE.repeat(2))?;
            }
        } else if index != accept_tab.len() - 1 {
            write!(file, ",{SPACE}")?;
        } else {
            writeln!(file)?;
        }
    }
    writeln!(file, "{SPACE}}} ;\n")?;

    Ok(accept_tab)
}
