mod file_parsing;
mod regex;

use file_parsing::parsing::parsing_lex_file;
use regex::{dfa::dfa::construct_dfa, nfa::nfa::construct_nfa, *};

fn main() {
    let args: Vec<String> = std::env::args().into_iter().collect();
    match parsing_lex_file(&args[1]) {
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
        _ => {}
    }

    let mut input: String = String::new();
    println!("Entre une regex: ");
    let result: Result<usize, std::io::Error> = std::io::stdin().read_line(&mut input);

    if result.is_err() {
        println!("error input");
    } else {
        println!("tu as tapee {}", input);
    }

    // Tokenizer
    let tokens: Vec<Token> = regex_tokenizer(&input);
    println!("les tokens sont == {:#?}", tokens);
    let (nfa, _) = construct_nfa(&tokens, 1);
    // // dbg!(&nfa);
    let _dfa = construct_dfa(nfa);
    // dbg!(&_dfa);

    println!("fini");
}
