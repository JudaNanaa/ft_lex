use std::{fs::File, io::Read, process::exit};

use crate::file_parsing::{combine::combine_nfa, rules::parse_rules_part};

use super::{definitions::parse_definitions_part, user_routine::parse_user_routine_part, FileInfo};

fn get_file_content(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut file_content = String::new();

    file.read_to_string(&mut file_content)?;
    // println!("{}", file_content);

    return Ok(file_content);
}

pub fn parsing_lex_file(file_path: &str) -> Result<(), String> {
    let file_content = match get_file_content(file_path) {
        Ok(content) => content,
        Err(_) => return Err("Cannot open file".to_string()),
    };
    let mut file = FileInfo {
        it: file_content.chars().peekable(),
        line_nb: 0,
        name: file_path,
    };

    let definitions = match parse_definitions_part(&mut file) {
        Ok(value) => value,
        Err(message) => {
            eprintln!("{}:{}: {}", file.name, file.line_nb, message);
            vec![] // a changer
        }
    };
    let rules = match parse_rules_part(&mut file, definitions) {
        Ok(value) => value,
        Err(message) => {
            eprintln!("{}:{}: {}", file.name, file.line_nb, message);
            exit(1); // TODO a changer
        }
    };

    dbg!(&rules.0);
	combine_nfa(rules.0);

    let user_routine = parse_user_routine_part(&mut file);

    //  TODO j'ai fait filePArt qui prends toutes les parties et va les return

    return Ok(());
}
