use std::{collections::HashMap, fs::File, io::Write};

use crate::lex_creation::SPACE;



pub fn create_yy_search_final(actions: &HashMap<usize, Vec<String>>, file: &mut File) -> std::io::Result<()> {

	writeln!(file, "void yy_search_final(int state, int len_match) {{")?;
	writeln!(file, "{}switch (state) {{", SPACE)?;
	
	for (nb, _) in actions {
		writeln!(file, "{}case {}:", SPACE.repeat(2), nb)?;
		writeln!(file, "{}final{}(len_match);", SPACE.repeat(3), nb)?;
		writeln!(file, "{}break;", SPACE.repeat(3))?;
	}
	writeln!(file, "{}default :", SPACE.repeat(2))?;
	writeln!(file, "{}yy_fatal_error(\"Not normal\");", SPACE.repeat(3))?;

	writeln!(file, "{}}}", SPACE)?;
	writeln!(file, "}}\n")?;
	
	Ok(())
}