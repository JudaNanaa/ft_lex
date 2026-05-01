use std::{fs::File, io::Read, io::stdin};

use crate::file_parsing::{
    definitions::{definitions::parse_definitions, Definition},
    rules::rules::{map_actions, parse_rules},
    user_routine::user_routine::parse_user_routine_part,
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

fn get_file_content(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut file_content = String::new();

    file.read_to_string(&mut file_content)?;

    Ok(file_content)
}

pub fn parsing_lex_file(file_path: &str) -> Result<FilePart, String> {
    let Ok(file_content) = get_file_content(file_path) else {
        return Err(format!("can't open {file_path}"));
    };

    let mut file = FileInfo {
        it: file_content.chars().peekable(),
        line_nb: 0,
        name: file_path,
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

fn get_stdin_content() -> std::io::Result<String> {
    let mut content = String::new();
    stdin().lock().read_to_string(&mut content)?;
    Ok(content)
}

pub fn parsing_lex_stdin() -> Result<FilePart, String> {
    let Ok(file_content) = get_stdin_content() else {
        return Err("can't read stdin".to_string());
    };

    let mut file = FileInfo {
        it: file_content.chars().peekable(),
        line_nb: 0,
        name: "<stdin>",
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
