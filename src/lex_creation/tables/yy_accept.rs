use crate::{lex_creation::SPACE, regex::dfa::Dfa};

pub fn compute_yy_accept(dfa: &Dfa) -> Vec<u8> {
    let nb_state = dfa.transitions().len();
    let mut tab = vec![0u8; nb_state];
    for state in dfa.transitions().keys() {
        if dfa.final_states().contains(state) {
            tab[*state] = 1;
        }
    }
    tab
}

pub fn write_yy_accept_c(tab: &[u8], file: &mut dyn std::io::Write) -> std::io::Result<()> {
    let nb_state = tab.len();
    writeln!(file, "\nconst int yy_accept[{nb_state}] =")?;
    writeln!(file, "{SPACE}{{")?;
    write!(file, "{}", SPACE.repeat(2))?;
    for (index, value) in tab.iter().enumerate() {
        write!(file, "{value}")?;
        if index != 0 && index % 10 == 0 {
            write!(file, ",")?;
            if index != tab.len() - 1 {
                write!(file, "\n{}", SPACE.repeat(2))?;
            }
        } else if index != tab.len() - 1 {
            write!(file, ",{SPACE}")?;
        } else {
            writeln!(file)?;
        }
    }
    writeln!(file, "{SPACE}}} ;\n")
}
