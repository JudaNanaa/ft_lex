use crate::{lex_creation::SPACE, regex::dfa::Dfa};

fn write_yy_nxt(
    transition_table: &[Vec<usize>],
    num_cols: usize,
    file: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    let nb_states = transition_table.len();
    let total = nb_states * num_cols;

    writeln!(file, "const int yy_nxt_cols = {num_cols};")?;
    writeln!(
        file,
        "#define YY_NXT(s, c) (yy_nxt_flat[(s) * yy_nxt_cols + (c)])"
    )?;
    writeln!(file, "\nconst unsigned int yy_nxt_flat[{total}] =")?;
    writeln!(file, "{SPACE}{{")?;

    let flat: Vec<usize> = transition_table.iter().flatten().copied().collect();
    let mut col = 0;
    write!(file, "{}", SPACE.repeat(2))?;
    for (i, &val) in flat.iter().enumerate() {
        write!(file, "{val}")?;
        if i + 1 < flat.len() {
            write!(file, ",")?;
            col += 1;
            if col % 10 == 0 {
                writeln!(file)?;
                write!(file, "{}", SPACE.repeat(2))?;
            } else {
                write!(file, "{SPACE}")?;
            }
        }
    }
    writeln!(file)?;
    writeln!(file, "{SPACE}}} ;\n")?;

    Ok(())
}

pub fn create_yy_nxt(dfa: &Dfa, file: &mut dyn std::io::Write) -> std::io::Result<Vec<Vec<usize>>> {
    let nb_states = dfa.transitions().iter().count();
    let num_classes = dfa.eq_classes.values().copied().max().unwrap_or(0);

    let mut transition_table: Vec<Vec<usize>> = Vec::with_capacity(nb_states);

    for i in 0..nb_states {
        let transitions = dfa.transitions().get(&i).unwrap();
        let mut tab = vec![0usize; num_classes + 1];
        for t in transitions {
            if let Some(&class_index) = dfa.eq_classes.get(t.input()) {
                tab[class_index] = *t.target_state();
            }
        }
        transition_table.push(tab);
    }

    write_yy_nxt(&transition_table, num_classes + 1, file)?;
    Ok(transition_table)
}
