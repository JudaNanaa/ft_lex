use std::collections::HashMap;
use std::io;

use crate::lex_creation::SPACE;

pub fn create_yy_ec(eq_classes: &HashMap<char, usize>, file: &mut dyn io::Write) -> io::Result<()> {
    writeln!(file, "\nconst unsigned char yy_ec[256] =")?;
    writeln!(file, "{SPACE}{{ 0,")?;
    write!(file, "{}", SPACE.repeat(2))?;

    for (i, byte) in (1u8..=255).enumerate() {
        let ch = byte as char;
        let class_index = eq_classes.get(&ch).copied().unwrap_or(0);
        write!(file, "{class_index}")?;

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

    Ok(())
}
