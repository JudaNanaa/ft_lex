use crate::lex_creation::SPACE;
use std::collections::HashMap;
use std::io;

pub fn compute_yy_ec(eq_classes: &HashMap<char, usize>) -> [usize; 256] {
    let mut ec = [0usize; 256];
    for byte in 1u8..=255 {
        let ch = byte as char;
        ec[byte as usize] = eq_classes.get(&ch).copied().unwrap_or(0);
    }
    ec
}

pub fn write_yy_ec_c(ec: &[usize; 256], file: &mut dyn io::Write) -> io::Result<()> {
    writeln!(file, "\nconst unsigned char yy_ec[256] =")?;
    writeln!(file, "{SPACE}{{ 0,")?;
    write!(file, "{}", SPACE.repeat(2))?;
    for (i, &val) in ec[1..].iter().enumerate() {
        write!(file, "{val}")?;
        if i + 1 < 255 {
            write!(file, ",")?;
            if (i + 1) % 10 == 0 {
                writeln!(file)?;
                write!(file, "{}", SPACE.repeat(2))?;
            } else {
                write!(file, "{SPACE}")?;
            }
        } else {
            writeln!(file, "\n{SPACE}}} ;\n")?;
        }
    }
    Ok(())
}
