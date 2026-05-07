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
        .map(|row| {
            if row.iter().any(|&v| v != 0) {
                1u8
            } else {
                0u8
            }
        })
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

pub struct YyNxtPackedData {
    pub base: Vec<usize>,
    pub nxt: Vec<usize>,
    pub chk: Vec<usize>,
}

pub fn pack_yy_nxt(data: &YyNxtData) -> YyNxtPackedData {
    let num_states = data.transition_table.len();
    let num_cols = data.num_cols;
    let sentinel = num_states;

    // Sort densest states first for better packing
    let mut order: Vec<usize> = (0..num_states).collect();
    order.sort_by_key(|&s| {
        std::cmp::Reverse(data.transition_table[s].iter().filter(|&&v| v != 0).count())
    });

    let mut packed_nxt: Vec<usize> = vec![0; num_cols.max(1)];
    let mut packed_chk: Vec<usize> = vec![sentinel; num_cols.max(1)];
    let mut base = vec![0usize; num_states];

    for &state in &order {
        let row = &data.transition_table[state];
        let non_zero: Vec<usize> = (0..num_cols).filter(|&ec| row[ec] != 0).collect();

        if non_zero.is_empty() {
            continue;
        }

        let mut offset = 0usize;
        'find: loop {
            let needed = offset + num_cols;
            if needed > packed_chk.len() {
                packed_nxt.resize(needed, 0);
                packed_chk.resize(needed, sentinel);
            }
            for &ec in &non_zero {
                if packed_chk[offset + ec] != sentinel {
                    offset += 1;
                    continue 'find;
                }
            }
            break;
        }

        base[state] = offset;

        let needed = offset + num_cols;
        if needed > packed_chk.len() {
            packed_nxt.resize(needed, 0);
            packed_chk.resize(needed, sentinel);
        }
        for &ec in &non_zero {
            packed_nxt[offset + ec] = row[ec];
            packed_chk[offset + ec] = state;
        }
    }

    YyNxtPackedData {
        base,
        nxt: packed_nxt,
        chk: packed_chk,
    }
}

pub fn write_yy_nxt_packed_c(
    data: &YyNxtPackedData,
    file: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    use crate::lex_creation::SPACE;

    let n = data.base.len();
    writeln!(file, "\nconst unsigned int yy_base[{n}] =")?;
    writeln!(file, "{SPACE}{{")?;
    write!(file, "{}", SPACE.repeat(2))?;
    for (i, &v) in data.base.iter().enumerate() {
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
    writeln!(file, "{SPACE}}} ;\n")?;

    let m = data.nxt.len();
    writeln!(file, "const unsigned int yy_nxt_packed[{m}] =")?;
    writeln!(file, "{SPACE}{{")?;
    write!(file, "{}", SPACE.repeat(2))?;
    for (i, &v) in data.nxt.iter().enumerate() {
        write!(file, "{v}")?;
        if i + 1 < m {
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
    writeln!(file, "{SPACE}}} ;\n")?;

    writeln!(file, "const unsigned int yy_chk[{m}] =")?;
    writeln!(file, "{SPACE}{{")?;
    write!(file, "{}", SPACE.repeat(2))?;
    for (i, &v) in data.chk.iter().enumerate() {
        write!(file, "{v}")?;
        if i + 1 < m {
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
    writeln!(file, "{SPACE}}} ;\n")?;

    writeln!(
        file,
        "#define YY_NXT(s, c) \\\n    (yy_chk[yy_base[(s)] + (c)] == (unsigned int)(s) \\\n        ? yy_nxt_packed[yy_base[(s)] + (c)] : 0)\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_data(rows: Vec<Vec<usize>>) -> YyNxtData {
        let num_cols = rows.first().map(|r| r.len()).unwrap_or(0);
        YyNxtData {
            transition_table: rows,
            num_cols,
        }
    }

    #[test]
    fn pack_small_table() {
        // 3 states, 3 cols
        // state 0: [1, 0, 2]
        // state 1: [0, 3, 0]
        // state 2: [0, 0, 0]
        let data = make_data(vec![vec![1, 0, 2], vec![0, 3, 0], vec![0, 0, 0]]);
        let packed = pack_yy_nxt(&data);

        assert_eq!(packed.base.len(), 3);

        // Verify round-trip: lookup should match original table
        for state in 0..3 {
            for ec in 0..3 {
                let expected = data.transition_table[state][ec];
                let pos = packed.base[state] + ec;
                let got = if pos < packed.chk.len() && packed.chk[pos] == state {
                    packed.nxt[pos]
                } else {
                    0
                };
                assert_eq!(got, expected, "state={state} ec={ec}");
            }
        }
    }

    #[test]
    fn pack_all_zero_table() {
        let data = make_data(vec![vec![0, 0], vec![0, 0]]);
        let packed = pack_yy_nxt(&data);
        // All lookups return 0
        for state in 0..2 {
            for ec in 0..2 {
                let pos = packed.base[state] + ec;
                let got = if pos < packed.chk.len() && packed.chk[pos] == state {
                    packed.nxt[pos]
                } else {
                    0
                };
                assert_eq!(got, 0);
            }
        }
    }
}
