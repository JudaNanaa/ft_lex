use crate::{lex_creation::SPACE, regex::dfa::Dfa};

pub struct YyNxtData {
    pub transition_table: Vec<Vec<usize>>,
    pub num_cols: usize,
}

pub fn compute_yy_nxt(dfa: &Dfa) -> YyNxtData {
    let nb_states = dfa.transitions().len();
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
    YyNxtData {
        transition_table,
        num_cols: num_classes + 1,
    }
}

pub fn write_yy_nxt_c(data: &YyNxtData, file: &mut dyn std::io::Write) -> std::io::Result<()> {
    let nb_states = data.transition_table.len();
    let total = nb_states * data.num_cols;
    writeln!(file, "const int yy_nxt_cols = {};", data.num_cols)?;
    writeln!(
        file,
        "#define YY_NXT(s, c) (yy_nxt_flat[(s) * yy_nxt_cols + (c)])"
    )?;
    writeln!(file, "\nconst unsigned int yy_nxt_flat[{total}] =")?;
    writeln!(file, "{SPACE}{{")?;
    let flat: Vec<usize> = data.transition_table.iter().flatten().copied().collect();
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
    writeln!(file, "{SPACE}}} ;\n")
}

pub fn compute_yy_has_trans(data: &YyNxtData) -> Vec<u8> {
    data.transition_table
        .iter()
        .map(|row| if row.iter().any(|&v| v != 0) { 1u8 } else { 0u8 })
        .collect()
}

pub fn write_yy_has_trans_c(
    has_trans: &[u8],
    file: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    use crate::lex_creation::SPACE;
    let n = has_trans.len();
    writeln!(file, "\nconst int yy_has_trans[{n}] =")?;
    writeln!(file, "{SPACE}{{")?;
    write!(file, "{}", SPACE.repeat(2))?;
    for (i, &v) in has_trans.iter().enumerate() {
        write!(file, "{v}")?;
        if i + 1 < n {
            write!(file, ",")?;
            if (i + 1) % 10 == 0 {
                writeln!(file)?;
                write!(file, "{}", SPACE.repeat(2))?;
            } else {
                write!(file, " ")?;
            }
        }
    }
    writeln!(file)?;
    writeln!(file, "{SPACE}}} ;\n")
}

pub fn write_yy_has_trans_rust(
    has_trans: &[u8],
    out: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    use crate::lex_creation::SPACE;
    writeln!(out, "static YY_HAS_TRANS: &[u8] = &[")?;
    write!(out, "{}", SPACE.repeat(2))?;
    for (i, &v) in has_trans.iter().enumerate() {
        write!(out, "{v}")?;
        if i + 1 < has_trans.len() {
            write!(out, ",")?;
            if (i + 1) % 16 == 0 {
                writeln!(out)?;
                write!(out, "{}", SPACE.repeat(2))?;
            } else {
                write!(out, " ")?;
            }
        }
    }
    writeln!(out)?;
    writeln!(out, "];")
}
