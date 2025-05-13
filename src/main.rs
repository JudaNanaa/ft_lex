mod file_parsing;
mod lex_creation;
mod regex;

use file_parsing::parsing::parsing_lex_file;
use lex_creation::creation::lex_creation;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_parts = match parsing_lex_file(&args[1]) {
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
        Ok(elems) => elems,
    };

    // dbg!(&file_parts);

    lex_creation(file_parts);
}
