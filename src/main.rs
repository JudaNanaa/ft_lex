mod regex;

use regex::{dfa::dfa::construct_dfa, nfa::nfa::construct_nfa, *};

fn main() {
    // loop {
        let mut input: String = String::new();
        // let result: Result<usize, std::io::Error> = std::io::stdin().read_line(&mut input);

        // if result.is_err() {
        //     println!("error input");
        // } else {
        //     println!("tu as tapee {}", input);
        // }
		input = ".*{100}".to_string();
        // Tokenizer
        let tokens: Vec<Token> = regex_tokenizer(&input);
        println!("les tokens sont == {:#?}", tokens);
        let nfa = construct_nfa(&tokens);
        // dbg!(&nfa);
        let _dfa = construct_dfa(nfa);
        // dbg!(&_dfa);

        println!("fini");
    // }
}
