use crate::{
    file_parsing::FilePart,
    lex_creation::{
        tables::{
            yy_accept::{compute_yy_accept, write_yy_accept_c},
            yy_ec::{compute_yy_ec, write_yy_ec_c},
            yy_nxt::{compute_yy_nxt, write_yy_nxt_c},
            yy_trailing::{compute_yy_trailing, write_yy_trailing_c},
            yy_trailing_accept::{compute_yy_trailing_accept, write_yy_trailing_accept_c},
        },
        SPACE,
    },
};

pub fn write_tables_c(file_parts: &FilePart, out: &mut dyn std::io::Write) -> std::io::Result<()> {
    write_yy_ec_c(&compute_yy_ec(&file_parts.dfa().eq_classes), out)?;
    let nxt = compute_yy_nxt(file_parts.dfa());
    write_yy_nxt_c(&nxt, out)?;
    write_yy_accept_c(&compute_yy_accept(file_parts.dfa()), out)?;
    write_yy_trailing_c(&compute_yy_trailing(file_parts.dfa()), out)?;
    write_yy_trailing_accept_c(&compute_yy_trailing_accept(file_parts.dfa()), out)
}

pub fn write_accept_actions_c(file_parts: &FilePart, out: &mut dyn std::io::Write) -> std::io::Result<()> {
    let final_states = file_parts.actions();
    let action_map = file_parts.map_actions();
    let nb_states = file_parts.dfa().transitions().len();
    let max_actions = final_states.values().map(|v| v.len()).max().unwrap_or(0);
    let cols = max_actions + 1;
    let mut flat = vec![0usize; nb_states * cols];
    for (state, actions) in final_states {
        for (i, action_str) in actions.iter().rev().enumerate() {
            let action_num = *action_map.get(action_str).unwrap();
            flat[state * cols + i] = action_num;
        }
    }
    writeln!(out, "const int yy_accept_cols = {cols};")?;
    writeln!(out, "#define YY_ACCEPT(s, i) (yy_accept_actions_flat[(s) * yy_accept_cols + (i)])")?;
    writeln!(out, "const int yy_accept_actions_flat[{}] =", nb_states * cols)?;
    writeln!(out, "{SPACE}{{")?;
    write!(out, "{}", SPACE.repeat(2))?;
    for (i, val) in flat.iter().enumerate() {
        write!(out, "{val}")?;
        if i == flat.len() - 1 {
            writeln!(out)?;
        } else {
            write!(out, ",")?;
            if (i + 1) % 10 == 0 {
                write!(out, "\n{}", SPACE.repeat(2))?;
            } else {
                write!(out, " ")?;
            }
        }
    }
    writeln!(out, "{SPACE}}};\n")
}
