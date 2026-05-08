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
    pub def: Vec<usize>,
}

pub fn pack_yy_nxt(data: &YyNxtData) -> YyNxtPackedData {
    let num_states = data.transition_table.len();
    let num_cols = data.num_cols;
    let jam = num_states;

    let mut order: Vec<usize> = (0..num_states).collect();
    order.sort_by_key(|&s| {
        std::cmp::Reverse(data.transition_table[s].iter().filter(|&&v| v != 0).count())
    });

    let mut packed_nxt: Vec<usize> = vec![0; num_cols.max(1)];
    let mut packed_chk: Vec<usize> = vec![jam; num_cols.max(1)];
    let mut base = vec![0usize; num_states];
    let mut def = vec![jam; num_states];
    let mut processed = vec![false; num_states];

    for &state in &order {
        let row = &data.transition_table[state];

        // Find the already-processed state with maximum column agreement
        let proto = (0..num_states)
            .filter(|&s| processed[s])
            .max_by_key(|&s| {
                let pr = &data.transition_table[s];
                row.iter().zip(pr.iter()).filter(|(&a, &b)| a == b).count()
            });

        def[state] = proto.unwrap_or(jam);

        let proto_row = proto
            .map(|p| data.transition_table[p].as_slice())
            .unwrap_or(&[]);

        // Store only entries where this state differs from its prototype
        let to_store: Vec<usize> = (0..num_cols)
            .filter(|&ec| row[ec] != proto_row.get(ec).copied().unwrap_or(0))
            .collect();

        processed[state] = true;

        if to_store.is_empty() {
            continue;
        }

        let mut offset = 0usize;
        'find: loop {
            let needed = offset + num_cols;
            if needed > packed_chk.len() {
                packed_nxt.resize(needed, 0);
                packed_chk.resize(needed, jam);
            }
            for &ec in &to_store {
                if packed_chk[offset + ec] != jam {
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
            packed_chk.resize(needed, jam);
        }
        for &ec in &to_store {
            packed_nxt[offset + ec] = row[ec];
            packed_chk[offset + ec] = state;
        }
    }

    YyNxtPackedData { base, nxt: packed_nxt, chk: packed_chk, def }
}

pub fn write_yy_nxt_packed_c(
    data: &YyNxtPackedData,
    file: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    use crate::lex_creation::SPACE;

    let n = data.base.len();
    let jam = n;

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

    writeln!(file, "const unsigned int yy_def[{n}] =")?;
    writeln!(file, "{SPACE}{{")?;
    write!(file, "{}", SPACE.repeat(2))?;
    for (i, &v) in data.def.iter().enumerate() {
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

    writeln!(file, "#define YY_JAM {jam}u\n")?;
    writeln!(
        file,
        "static inline unsigned int yy_nxt_lookup(unsigned int state, unsigned int ec) {{"
    )?;
    writeln!(file, "{SPACE}while (state != YY_JAM) {{")?;
    writeln!(
        file,
        "{SPACE}{SPACE}unsigned int pos = yy_base[state] + ec;"
    )?;
    writeln!(
        file,
        "{SPACE}{SPACE}if (yy_chk[pos] == state) return yy_nxt_packed[pos];"
    )?;
    writeln!(file, "{SPACE}{SPACE}state = yy_def[state];")?;
    writeln!(file, "{SPACE}}}")?;
    writeln!(file, "{SPACE}return 0;")?;
    writeln!(file, "}}")?;
    writeln!(file, "#define YY_NXT(s, c) yy_nxt_lookup((s), (c))\n")
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

    fn lookup(packed: &YyNxtPackedData, state: usize, ec: usize) -> usize {
        let jam = packed.base.len();
        let mut s = state;
        loop {
            if s == jam {
                return 0;
            }
            let pos = packed.base[s] + ec;
            if pos < packed.chk.len() && packed.chk[pos] == s {
                return packed.nxt[pos];
            }
            s = packed.def[s];
        }
    }

    #[test]
    fn pack_small_table() {
        let data = make_data(vec![vec![1, 0, 2], vec![0, 3, 0], vec![0, 0, 0]]);
        let packed = pack_yy_nxt(&data);

        assert_eq!(packed.base.len(), 3);
        assert_eq!(packed.def.len(), 3);

        for state in 0..3 {
            for ec in 0..3 {
                let expected = data.transition_table[state][ec];
                let got = lookup(&packed, state, ec);
                assert_eq!(got, expected, "state={state} ec={ec}");
            }
        }
    }

    #[test]
    fn pack_all_zero_table() {
        let data = make_data(vec![vec![0, 0], vec![0, 0]]);
        let packed = pack_yy_nxt(&data);
        assert_eq!(packed.def.len(), 2);
        for state in 0..2 {
            for ec in 0..2 {
                assert_eq!(lookup(&packed, state, ec), 0);
            }
        }
    }

    #[test]
    fn pack_prototype_sharing() {
        // state 0: dense prototype [1, 2, 3, 4]
        // state 1: differs only at ec=3 → [1, 2, 3, 0]
        // With prototype chaining, state 1 uses state 0 as its prototype and
        // stores only 1 entry (ec=3, nxt=0) instead of 3 entries (old algo).
        let data = make_data(vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 0],
        ]);
        let packed = pack_yy_nxt(&data);

        for state in 0..2 {
            for ec in 0..4 {
                assert_eq!(
                    lookup(&packed, state, ec),
                    data.transition_table[state][ec],
                    "state={state} ec={ec}"
                );
            }
        }

        // state 1's prototype should be state 0 (3 out of 4 columns agree)
        assert_eq!(packed.def[1], 0, "state 1 should have state 0 as prototype");

        // With sharing: state0 stores 4 entries, state1 stores 1.
        // nxt[] needs at most 5 filled slots → length ≤ 6 (with possible offset gap).
        assert!(
            packed.nxt.len() <= 6,
            "expected prototype sharing to compress nxt[], got len={}",
            packed.nxt.len()
        );
    }
}
