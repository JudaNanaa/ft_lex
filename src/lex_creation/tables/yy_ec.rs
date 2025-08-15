use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Write,
};

use crate::lex_creation::SPACE;

pub fn create_yy_ec(
    charset: &HashSet<char>,
    file: &mut File,
) -> std::io::Result<HashMap<char, usize>> {
    let mut hash = HashMap::new();
    let mut eq_index = 1;

    writeln!(file, "\nconst unsigned char yy_ec[256] =")?;
    writeln!(file, "{}{{ 0,", SPACE)?;
    write!(file, "{}", SPACE.repeat(2))?;

    for (i, byte) in (1u8..=255).enumerate() {
        let ch = byte as char;
        if charset.contains(&ch) {
            write!(file, "{}", eq_index)?;
            hash.insert(ch, eq_index);
            eq_index += 1;
        } else {
            write!(file, "0")?;
        }

        if byte != 255 {
            write!(file, ",")?;
            if (i + 1) % 10 == 0 {
                writeln!(file)?;
                write!(file, "{}", SPACE.repeat(2))?;
            } else {
                write!(file, "{}", SPACE)?;
            }
        } else {
            writeln!(file, "\n{}}} ;\n", SPACE.to_string())?;
        }
    }

    return Ok(hash);
}
