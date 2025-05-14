use std::{fs::File, io::Write};

use crate::{file_parsing::FilePart, lex_creation::SPACE};


pub fn yy_action(file_parts: &FilePart, file: &mut File) -> std::io::Result<()> {

	let action_hash = file_parts.action_hash();

	writeln!(file, "void yy_action(int action) {{")?;
	writeln!(file, "{}switch (action) {{", SPACE)?;
	
	for nb_action in 1..=action_hash.len() {
		writeln!(file, "{}case {}:", SPACE.repeat(2), nb_action)?;
		writeln!(file, "{}", action_hash.)?;
	}
	writeln!(file, "{}default:", SPACE.repeat(2))?;
	writeln!(file, "{}yy_fatal_error(\"not normal\");", SPACE.repeat(3))?;
	
	writeln!(file, "{}}}", SPACE)?;
	
	writeln!(file, "}}")?;

	todo!();
}