use std::{fs::File, io::Read, process::exit};

use crate::file_parsing::{definitions::definitions::parse_definitions_part, rules::rules::{action_hash, parse_rules_section}, user_routine::user_routine::parse_user_routine_part, FilePart};

use super::{
    combine::process_and_combine_rules,
    FileInfo,
};

fn get_file_content(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut file_content = String::new();

    file.read_to_string(&mut file_content)?;

    return Ok(file_content);
}

pub fn parsing_lex_file(file_path: &str) -> Result<FilePart, &'static str> {
    let file_content = match get_file_content(file_path) {
        Ok(content) => content,
        Err(_) => return Err("Cannot open file"),
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

    let (rules, in_yylex) = match parse_rules_section(&mut file, &definitions) {
        Ok(value) => value,
        Err(message) => {
            eprintln!("{}:{}: {}", file.name, file.line_nb, message);
            exit(1); // TODO a changer
        }
    };

    let action_hash = action_hash(&rules);

    let (dfa, actions) = process_and_combine_rules(rules)?;

    let user_routine = parse_user_routine_part(&mut file);

    return Ok(FilePart {
        definitions,
        in_yylex,
        dfa,
        actions,
        action_hash,
        user_routine,
    });
}
