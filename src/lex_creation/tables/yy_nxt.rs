use std::{collections::HashMap, fs::File, io::Write};

use crate::{lex_creation::SPACE, regex::dfa::DFA};

fn write_yy_nxt(transition_table: &Vec<Vec<usize>>, file: &mut File) -> std::io::Result<()> {
    writeln!(
        file,
        "\nconst unsigned int yy_nxt[{}][{}] =",
        transition_table.len(),
        transition_table[0].len()
    )?;
    writeln!(file, "{}{{", SPACE)?;

    for (index, state) in transition_table.iter().enumerate() {
        writeln!(file, "{}{{", SPACE.repeat(2))?;
        write!(file, "{}", SPACE.repeat(3))?;
        for (index, elem) in state.iter().enumerate() {
            write!(file, "{}", elem)?;
            if index != 0 && index % 10 == 0 {
                write!(file, ",")?;
                if index != state.len() - 1 {
                    write!(file, "\n{}", SPACE.repeat(2))?;
                }
            } else if index != state.len() - 1 {
                write!(file, ",{}", SPACE)?;
            } else {
                writeln!(file, "")?;
            }
        }
        if index != transition_table.len() - 1 {
            writeln!(file, "{}}},", SPACE.repeat(2))?;
        } else {
            writeln!(file, "{}}}", SPACE.repeat(2))?;
        }
    }
    writeln!(file, "{}}} ;\n", SPACE)?;

    return Ok(());
}

pub fn create_yy_nxt(
    dfa: &DFA,
    hash: &HashMap<char, usize>,
    file: &mut File,
) -> std::io::Result<Vec<Vec<usize>>> {
    let nb_states = dfa.new_transitions().iter().count();

    let mut transition_table: Vec<Vec<usize>> = Vec::with_capacity(nb_states);

    let nb_possibilities = hash.len() + 1;

    for i in 0..nb_states {
        let transitions = dfa.new_transitions().get(&i).unwrap();
        let mut tab = vec![0; 256];
        for t in transitions {
            if let Some(index) = hash.get(t.input()) {
                tab[*index] = *t.target_state();
            }
        }
        transition_table.push(tab);
    }

    write_yy_nxt(&transition_table, file)?;
    return Ok(transition_table);
}
