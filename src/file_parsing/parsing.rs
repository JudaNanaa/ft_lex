use std::{fs::File, io::stdin, io::Read};

use crate::file_parsing::{
    definitions::{parse::parse_definitions, Definition},
    rules::parse::{map_actions, parse_rules},
    user_routine::parse::parse_user_routine_part,
    FilePart, YytextMode,
};

use super::{combine::process_and_combine_rules, FileInfo};

pub(super) fn extract_yytext_mode(definitions: &[Definition]) -> YytextMode {
    let mut mode = YytextMode::Pointer;
    for def in definitions {
        if let Definition::Option { name } = def {
            if name == "array" {
                mode = YytextMode::Array(8192);
            } else if name == "pointer" {
                mode = YytextMode::Pointer;
            } else if let Some(val) = name.strip_prefix("yylmax=") {
                if let Ok(n) = val.parse::<usize>() {
                    mode = YytextMode::Array(n);
                }
            }
        }
    }
    mode
}

pub fn get_file_content(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn get_stdin_content() -> std::io::Result<String> {
    let mut content = String::new();
    stdin().lock().read_to_string(&mut content)?;
    Ok(content)
}

pub fn parse_lex_content(content: &str, name: &str) -> Result<FilePart, String> {
    let mut file = FileInfo {
        it: content.chars().peekable(),
        line_nb: 0,
        name,
    };

    let definitions = match parse_definitions(&mut file) {
        Ok(value) => value,
        Err(message) => return Err(format!("{}:{}: {}", file.name, file.line_nb, message)),
    };

    let (rules, in_yylex) = match parse_rules(&mut file, &definitions) {
        Ok(value) => value,
        Err(message) => return Err(format!("{}:{}: {}", file.name, file.line_nb, message)),
    };

    let map_actions = map_actions(&rules);
    let (dfa, actions, rule_action) = process_and_combine_rules(rules)?;
    let user_routine = parse_user_routine_part(&mut file);
    let yytext_mode = extract_yytext_mode(&definitions);

    Ok(FilePart {
        definitions,
        in_yylex,
        dfa,
        rule_action,
        actions,
        map_actions,
        user_routine,
        yytext_mode,
    })
}
