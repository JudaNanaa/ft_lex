mod file_parsing;
mod lex_creation;
mod regex;

use file_parsing::parsing::{get_file_content, get_stdin_content, parse_lex_content};
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

    let mut combined_content = String::new();
    let mut names: Vec<String> = Vec::new();

    for source in &sources {
        let (content, name) = match source {
            None => {
                match get_stdin_content() {
                    Err(_) => {
                        eprintln!("ft_lex: can't read stdin");
                        return;
                    }
                    Ok(c) => (c, "<stdin>".to_string()),
                }
            }
            Some(path) => {
                match get_file_content(path) {
                    Err(_) => {
                        eprintln!("ft_lex: can't open {}", path);
                        return;
                    }
                    Ok(c) => (c, path.clone()),
                }
            }
        };
        if !combined_content.is_empty() {
            combined_content.push('\n');
        }
        combined_content.push_str(&content);
        names.push(name);
    }

    let parse_name = if names.len() == 1 {
        names.remove(0)
    } else {
        names.join(" ")
    };

    let file_parts = match parse_lex_content(&combined_content, &parse_name) {
        Err(error) => {
            eprintln!("ft_lex: {}", error);
            return;
        }
        Ok(parts) => parts,
    };

    match lex_creation(&file_parts) {
        Ok(()) => {}
        Err(error) => eprintln!("Error: {}", error),
    }
}
