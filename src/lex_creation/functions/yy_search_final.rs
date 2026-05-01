use std::collections::HashMap;

use crate::lex_creation::SPACE;

pub fn create_yy_search_final(
    actions: &HashMap<usize, Vec<String>>,
    file: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    writeln!(file, "void yy_search_final(int state, int len_match) {{")?;
    writeln!(file, "{SPACE}switch (state) {{")?;

    for nb in actions.keys() {
        writeln!(file, "{}case {}:", SPACE.repeat(2), nb)?;
        writeln!(file, "{}final{}(len_match);", SPACE.repeat(3), nb)?;
        writeln!(file, "{}break;", SPACE.repeat(3))?;
    }
    writeln!(file, "{}default :", SPACE.repeat(2))?;
    writeln!(file, "{}yy_fatal_error(\"Not normal\");", SPACE.repeat(3))?;

    writeln!(file, "{SPACE}}}")?;
    writeln!(file, "}}\n")?;

    Ok(())
}
