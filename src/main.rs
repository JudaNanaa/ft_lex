mod file_parsing;
mod lex_creation;
mod regex;

use file_parsing::merge::merge_file_parts;
use file_parsing::parsing::{parsing_lex_file, parsing_lex_stdin};
use lex_creation::creation::lex_creation;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let sources: Vec<Option<String>> = if args.is_empty() {
        vec![None]
    } else {
        args.iter()
            .map(|a| if a == "-" { None } else { Some(a.clone()) })
            .collect()
    };

    let mut parts = Vec::new();

    for source in sources {
        let result = match source {
            None => parsing_lex_stdin(),
            Some(ref path) => parsing_lex_file(path),
        };
        match result {
            Err(error) => {
                eprintln!("ft_lex: {}", error);
                return;
            }
            Ok(part) => parts.push(part),
        }
    }

    let file_parts = if parts.len() == 1 {
        parts.remove(0)
    } else {
        match merge_file_parts(parts) {
            Err(error) => {
                eprintln!("ft_lex: {}", error);
                return;
            }
            Ok(merged) => merged,
        }
    };

    match lex_creation(&file_parts) {
        Ok(()) => {}
        Err(error) => eprintln!("Error: {}", error),
    }
}
