use crate::{
    file_parsing::FilePart,
    lex_creation::{
        tables::{
            yy_accept::compute_yy_accept,
            yy_ec::compute_yy_ec,
            yy_nxt::compute_yy_nxt,
            yy_trailing::compute_yy_trailing,
            yy_trailing_accept::compute_yy_trailing_accept,
        },
        SPACE,
    },
};

fn write_usize_slice(name: &str, data: &[usize], out: &mut dyn std::io::Write) -> std::io::Result<()> {
    writeln!(out, "static {name}: &[usize] = &[")?;
    write!(out, "{}", SPACE.repeat(2))?;
    for (i, val) in data.iter().enumerate() {
        write!(out, "{val}")?;
        if i + 1 < data.len() {
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

fn write_u8_slice(name: &str, data: &[u8], out: &mut dyn std::io::Write) -> std::io::Result<()> {
    writeln!(out, "static {name}: &[u8] = &[")?;
    write!(out, "{}", SPACE.repeat(2))?;
    for (i, val) in data.iter().enumerate() {
        write!(out, "{val}")?;
        if i + 1 < data.len() {
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

pub fn write_tables_rust(file_parts: &FilePart, out: &mut dyn std::io::Write) -> std::io::Result<()> {
    let nxt = compute_yy_nxt(file_parts.dfa());
    let flat: Vec<usize> = nxt.transition_table.iter().flatten().copied().collect();
    writeln!(out, "const YY_NXT_COLS: usize = {};", nxt.num_cols)?;
    write_usize_slice("YY_NXT_FLAT", &flat, out)?;
    writeln!(out)?;

    let ec = compute_yy_ec(&file_parts.dfa().eq_classes);
    let ec_slice: Vec<u8> = ec.iter().map(|&v| v as u8).collect();
    write_u8_slice("YY_EC", &ec_slice, out)?;
    writeln!(out)?;

    write_u8_slice("YY_ACCEPT", &compute_yy_accept(file_parts.dfa()), out)?;
    writeln!(out)?;
    write_u8_slice("YY_TRAILING", &compute_yy_trailing(file_parts.dfa()), out)?;
    writeln!(out)?;
    write_u8_slice("YY_TRAILING_ACCEPT", &compute_yy_trailing_accept(file_parts.dfa()), out)?;
    writeln!(out)
}

pub fn write_accept_actions_rust(file_parts: &FilePart, out: &mut dyn std::io::Write) -> std::io::Result<()> {
    let final_states = file_parts.actions();
    let action_map = file_parts.map_actions();
    let nb_states = file_parts.dfa().transitions().len();
    let max_actions = final_states.values().map(|v| v.len()).max().unwrap_or(0);
    let cols = max_actions + 1;
    let mut flat = vec![0usize; nb_states * cols];
    for (state, actions) in final_states {
        for (i, action_str) in actions.iter().rev().enumerate() {
            flat[state * cols + i] = *action_map.get(action_str).unwrap();
        }
    }
    writeln!(out, "const YY_ACCEPT_COLS: usize = {cols};")?;
    write_usize_slice("YY_ACCEPT_FLAT", &flat, out)?;
    writeln!(out)
}
