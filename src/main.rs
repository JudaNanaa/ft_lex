mod regex;

use regex::{nfa::nfa::construct_nfa, *};

fn main() {
    loop {
        let mut input: String = String::new();
        let result: Result<usize, std::io::Error> = std::io::stdin().read_line(&mut input);

        if result.is_err() {
            println!("error input");
        } else {
            println!("tu as tapee {}", input);
        }

        // Tokenizer
        let tokens: Vec<Token> = regex_tokenizer(&input);
        let nfa = construct_nfa(&tokens);
        dbg!(&nfa);
    }
}
