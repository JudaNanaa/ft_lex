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

enum OutputDest {
    File,
    Stdout,
}
enum Backend {
    C,
    Rust,
}

struct Opts {
    output: OutputDest,
    backend: Backend,
    verbose: bool,
    no_stats: bool,
    compressed: bool,
    sources: Vec<Option<String>>,
}

fn parse_args(args: &[String]) -> Opts {
    let mut output = OutputDest::File;
    let mut backend = Backend::C;
    let mut verbose = false;
    let mut no_stats = false;
    let mut compressed = false;
    let mut sources: Vec<Option<String>> = vec![];
    let mut stdin_seen = false;
    if args.is_empty() {
        sources.push(None);
        return Opts {
            output,
            backend,
            verbose,
            no_stats,
            compressed,
            sources,
        };
    }
    for a in args {
        match a.as_str() {
            "-t" => output = OutputDest::Stdout,
            "-v" => verbose = true,
            "-n" => no_stats = true,
            "-C" => compressed = true,
            "--rust" => backend = Backend::Rust,
            "-" if !stdin_seen => {
                stdin_seen = true;
                sources.push(None);
            }
            "-" => {}
            _ if a.starts_with('-') => {}
            _ => sources.push(Some(a.clone())),
        }
    }
    Opts {
        output,
        backend,
        verbose,
        no_stats,
        compressed,
        sources,
    }
}

fn read_sources(sources: &[Option<String>]) -> Option<(String, String)> {
    let mut combined = String::new();
    let mut names: Vec<String> = Vec::new();
    for source in sources {
        let (content, name) = match source {
            None => {
                if let Ok(c) = get_stdin_content() {
                    (c, "<stdin>".to_string())
                } else {
                    eprintln!("ft_lex: can't read stdin");
                    return None;
                }
            }
            Some(path) => {
                if let Ok(c) = get_file_content(path) {
                    (c, path.clone())
                } else {
                    eprintln!("ft_lex: can't open {path}");
                    return None;
                }
            }
        };
        combined.push_str(&content);
        names.push(name);
    }
    let name = if names.len() == 1 {
        names.remove(0)
    } else {
        names.join(" ")
    };
    Some((combined, name))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let opts = parse_args(&args);

    let Some((content, name)) = read_sources(&opts.sources) else {
        return;
    };

    let file_parts = match parse_lex_content(&content, &name) {
        Ok(parts) => parts,
        Err(e) => {
            eprintln!("ft_lex: {e}");
            return;
        }
    };

    let backend: Box<dyn CodegenBackend> = match opts.backend {
        Backend::Rust => Box::new(lex_creation::rust::RustBackend::new(opts.compressed)),
        Backend::C => Box::new(CBackend::new(opts.compressed)),
    };

    let result = match opts.output {
        OutputDest::Stdout => lex_creation(&file_parts, backend.as_ref(), &mut std::io::stdout()),
        OutputDest::File => {
            let filename = backend.output_filename();
            if let Ok(mut f) = File::create(filename) {
                lex_creation(&file_parts, backend.as_ref(), &mut f)
            } else {
                eprintln!("ft_lex: can't create {filename}");
                return;
            }
        }
    };

    if let Err(e) = result {
        if e.kind() != std::io::ErrorKind::BrokenPipe {
            eprintln!("Error: {e}");
        }
    }

    if opts.verbose && !opts.no_stats {
        let out: &mut dyn std::io::Write = match opts.output {
            OutputDest::Stdout => &mut std::io::stderr(),
            OutputDest::File => &mut std::io::stdout(),
        };
        print_stats(&compute_stats(&file_parts, opts.compressed), out);
    }
}
