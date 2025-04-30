use std::{collections::{HashMap, HashSet}, fs::File, io::Write};

use crate::{file_parsing::FilePart, regex::dfa::DFA};

const SPACE: &str = "    ";

fn create_yy_ec(charset: &HashSet<char>, file: &mut File) -> std::io::Result<HashMap<char, usize>> {
    let mut hash = HashMap::new();
    let mut eq_index = 2;

    writeln!(file, "\nstatic const unsigned char yy_ec[256] =")?;
    writeln!(file, "{}{{ 0,", SPACE)?;
    write!(file, "{}", SPACE.repeat(2))?;

    for (i, byte) in (1u8..=255).enumerate() {
        let ch = byte as char;
        if charset.contains(&ch) {
            write!(file, "{}", eq_index)?;
            hash.insert(ch, eq_index);
            eq_index += 1;
        } else {
            write!(file, "1")?;
        }

        if byte != 255 {
            write!(file, ",")?;
            if (i + 1) % 10 == 0 {
                writeln!(file)?;
                write!(file, "{}{}", SPACE.repeat(2), "")?;
            } else {
                write!(file, "{}", SPACE)?;
            }
        } else {
            writeln!(file, "\n{}}} ;", SPACE.repeat(1))?;
        }
    }

    return Ok(hash);
}

fn create_yy_nxt(dfa: &DFA, hash: &HashMap<char, usize>, file: &mut File) -> std::io::Result<()> {

	let nb_states = dfa.new_transitions().iter().count();
	dbg!(nb_states);

	for i in 0..nb_states {
		let transitions = dfa.new_transitions().get(&i).unwrap();
		dbg!(transitions);
	}

	todo!();
}

pub fn tables_creation(file_parts: &FilePart, file: &mut File) -> std::io::Result<()> {

	let eq_hash = create_yy_ec(file_parts.dfa().charset(), file)?;

	create_yy_nxt(file_parts.dfa(), &eq_hash, file)?;
    return Ok(());
}
