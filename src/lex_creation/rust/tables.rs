use crate::{
    file_parsing::FilePart,
    lex_creation::{
        tables::{
            yy_accept::compute_yy_accept, yy_ec::compute_yy_ec,
            yy_trailing::compute_yy_trailing, yy_trailing_accept::compute_yy_trailing_accept,
        },
        SPACE,
    },
};

pub fn write_usize_slice_pub(
    name: &str,
    data: &[usize],
    out: &mut dyn std::io::Write,
) -> std::io::Result<()> {
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

pub fn write_tables_rust(
    file_parts: &FilePart,
    compressed: bool,
    out: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    use crate::lex_creation::tables::yy_nxt::{
        compute_yy_has_trans, compute_yy_nxt, write_yy_has_trans_rust,
    };
    let _ = compressed; // will be used in Task 7

    let nxt = compute_yy_nxt(file_parts.dfa());
    let flat: Vec<usize> = nxt.transition_table.iter().flatten().copied().collect();
    writeln!(out, "const YY_NXT_COLS: usize = {};", nxt.num_cols)?;
    write_usize_slice_pub("YY_NXT_FLAT", &flat, out)?;
    writeln!(out)?;

    let ec = compute_yy_ec(&file_parts.dfa().eq_classes);
    let ec_slice: Vec<u8> = ec
        .iter()
        .map(|&v| u8::try_from(v).expect("EC value exceeds u8"))
        .collect();
    write_u8_slice("YY_EC", &ec_slice, out)?;
    writeln!(out)?;

    write_yy_has_trans_rust(&compute_yy_has_trans(&nxt), out)?;
    writeln!(out)?;

    write_u8_slice("YY_ACCEPT", &compute_yy_accept(file_parts.dfa()), out)?;
    writeln!(out)?;
    write_u8_slice("YY_TRAILING", &compute_yy_trailing(file_parts.dfa()), out)?;
    writeln!(out)?;
    write_u8_slice(
        "YY_TRAILING_ACCEPT",
        &compute_yy_trailing_accept(file_parts.dfa()),
        out,
    )?;
    writeln!(out)
}
