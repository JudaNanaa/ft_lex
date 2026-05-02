use std::collections::{HashMap, HashSet};

use crate::lex_creation::SPACE;

pub fn create_yy_ec(
    charset: &HashSet<char>,
    file: &mut dyn std::io::Write,
) -> std::io::Result<HashMap<char, usize>> {
    let mut hash = HashMap::new();
    let mut eq_index = 1;

    writeln!(file, "\nconst unsigned char yy_ec[256] =")?;
    writeln!(file, "{SPACE}{{ 0,")?;
    write!(file, "{}", SPACE.repeat(2))?;

    for (i, byte) in (1u8..=255).enumerate() {
        let ch = byte as char;
        if charset.contains(&ch) {
            write!(file, "{eq_index}")?;
            hash.insert(ch, eq_index);
            eq_index += 1;
        } else {
            write!(file, "0")?;
        }

        if byte == 255 {
            writeln!(file, "\n{SPACE}}} ;\n")?;
        } else {
            write!(file, ",")?;
            if (i + 1) % 10 == 0 {
                writeln!(file)?;
                write!(file, "{}", SPACE.repeat(2))?;
            } else {
                write!(file, "{SPACE}")?;
            }
        }
    }

    Ok(hash)
}
