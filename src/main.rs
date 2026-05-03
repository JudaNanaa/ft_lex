mod file_parsing;
mod lex_creation;
mod regex;

use std::fs::File;

use file_parsing::parsing::{get_file_content, get_stdin_content, parse_lex_content};
use lex_creation::{
    backend::CodegenBackend,
    c::CBackend,
    creation::lex_creation,
    stats::{compute_stats, print_stats},
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut flag_t = false;
    let mut flag_v = false;
    let mut flag_n = false;
    let mut flag_rust = false;
    let mut stdin_seen = false;

    let sources: Vec<Option<String>> = if args.is_empty() {
        vec![None]
    } else {
        args.iter()
            .filter_map(|a| match a.as_str() {
                "-t" => {
                    flag_t = true;
                    None
                }
                "-v" => {
                    flag_v = true;
                    None
                }
                "-n" => {
                    flag_n = true;
                    None
                }
                "--rust" => {
                    flag_rust = true;
                    None
                }
                "-" => {
                    if stdin_seen {
                        return None;
                    }
                    stdin_seen = true;
                    Some(None)
                }
                _ if a.starts_with('-') => None,
                _ => Some(Some(a.clone())),
            })
            .collect()
    };

    let mut combined_content = String::new();
    let mut names: Vec<String> = Vec::new();

    for source in &sources {
        let (content, name) = match source {
            None => match get_stdin_content() {
                Err(_) => {
                    eprintln!("ft_lex: can't read stdin");
                    return;
                }
                Ok(c) => (c, "<stdin>".to_string()),
            },
            Some(path) => match get_file_content(path) {
                Err(_) => {
                    eprintln!("ft_lex: can't open {path}");
                    return;
                }
                Ok(c) => (c, path.clone()),
            },
        };
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
            eprintln!("ft_lex: {error}");
            return;
        }
        Ok(parts) => parts,
    };

    let backend: Box<dyn CodegenBackend> = if flag_rust {
        Box::new(lex_creation::rust::RustBackend::new())
    } else {
        Box::new(CBackend::new())
    };

    let result = if flag_t {
        lex_creation(&file_parts, backend.as_ref(), &mut std::io::stdout())
    } else {
        let filename = backend.output_filename();
        match File::create(filename) {
            Err(_) => {
                eprintln!("ft_lex: can't create {filename}");
                return;
            }
            Ok(mut f) => lex_creation(&file_parts, backend.as_ref(), &mut f),
        }
    };

    if let Err(error) = result {
        if error.kind() != std::io::ErrorKind::BrokenPipe {
            eprintln!("Error: {error}");
        }
    }

    if flag_v && !flag_n {
        if flag_t {
            print_stats(&compute_stats(&file_parts), &mut std::io::stderr());
        } else {
            print_stats(&compute_stats(&file_parts), &mut std::io::stdout());
        }
    }
}
