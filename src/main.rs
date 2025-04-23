mod file_parsing;
mod regex;

use file_parsing::parsing::parsing_lex_file;
use regex::{dfa::dfa::construct_dfa, nfa::nfa::construct_nfa, *};

fn main() {
    let args: Vec<String> = std::env::args().into_iter().collect();
    let file_parts = match parsing_lex_file(&args[1]) {
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
        Ok(elems) => {
			elems
		}
    };

	dbg!(file_parts);
}
