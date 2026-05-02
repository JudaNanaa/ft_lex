use crate::{file_parsing::FilePart, lex_creation::SPACE};

pub fn write_yy_accept_actions(
    file_parts: &FilePart,
    file: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    let final_states = file_parts.actions();
    let action_map = file_parts.map_actions();
    let nb_states = file_parts.dfa().transitions().len();

    let max_actions = final_states
        .values()
        .map(std::vec::Vec::len)
        .max()
        .unwrap_or(0);
    let cols = max_actions + 1;

    let mut flat = vec![0usize; nb_states * cols];
    for (state, actions) in final_states {
        for (i, action_str) in actions.iter().rev().enumerate() {
            let action_num = *action_map.get(action_str).unwrap();
            flat[state * cols + i] = action_num;
        }
    }

    writeln!(file, "const int yy_accept_cols = {cols};")?;
    writeln!(
        file,
        "#define YY_ACCEPT(s, i) (yy_accept_actions_flat[(s) * yy_accept_cols + (i)])"
    )?;
    writeln!(
        file,
        "const int yy_accept_actions_flat[{}] =",
        nb_states * cols
    )?;
    writeln!(file, "{SPACE}{{")?;
    write!(file, "{}", SPACE.repeat(2))?;
    for (i, val) in flat.iter().enumerate() {
        write!(file, "{val}")?;
        if i == flat.len() - 1 {
            writeln!(file)?;
        } else {
            write!(file, ",")?;
            if (i + 1) % 10 == 0 {
                write!(file, "\n{}", SPACE.repeat(2))?;
            } else {
                write!(file, " ")?;
            }
        }
    }
    writeln!(file, "{SPACE}}};\n")?;

    Ok(())
}
