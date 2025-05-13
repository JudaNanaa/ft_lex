use std::{collections::HashMap, fs::File, io::Write};

use super::SPACE;



pub fn add_action(actions: &HashMap<usize, Vec<String>>, file: &mut File) -> std::io::Result<()> {

	for (number, list) in actions {
		writeln!(file, "void action{}(void) {{", number)?;
		
		for action in list {
			writeln!(file, "{}{}", SPACE, action)?;
		}

		writeln!(file, "}}")?;
	}

	Ok(())
}