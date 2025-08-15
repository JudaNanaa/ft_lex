use crate::regex::{nfa::nfa::construct_nfa, Operator, Quantifier, Token};

use super::regex_tokenizer;

#[test]
fn test_regex_1() {
    let tokens = regex_tokenizer(&"chef".to_string());

    assert_eq!(tokens[0], Token::Char('c'));
    assert_eq!(tokens[1], Token::Char('h'));
    assert_eq!(tokens[2], Token::Operator(Operator::Concatenation));
    assert_eq!(tokens[3], Token::Char('e'));
    assert_eq!(tokens[4], Token::Operator(Operator::Concatenation));
    assert_eq!(tokens[5], Token::Char('f'));
    assert_eq!(tokens[6], Token::Operator(Operator::Concatenation));

    let nfa = construct_nfa(&tokens, 1);

    assert_eq!(nfa.final_states.len(), 1);
}

#[test]
fn test_regex_2() {
    let tokens = regex_tokenizer(&"a|b".to_string());

    assert_eq!(tokens[0], Token::Char('a'));
    assert_eq!(tokens[1], Token::Char('b'));
    assert_eq!(tokens[2], Token::Operator(Operator::Or));

    let nfa = construct_nfa(&tokens, 1);

    assert_eq!(nfa.final_states.len(), 2);
}

#[test]
fn test_regex_3() {
    let tokens = regex_tokenizer(&"a*".to_string());

    assert_eq!(tokens[0], Token::Char('a'));
    assert_eq!(
        tokens[1],
        Token::Operator(Operator::Quantifier(Quantifier::AtLeast(0)))
    );

    let nfa = construct_nfa(&tokens, 1);

    assert_eq!(nfa.transitions.len(), 2);
    assert_eq!(nfa.final_states.len(), 2);
}

#[test]
fn test_regex_group_concatenation() {
    let tokens = regex_tokenizer(&"(ab)c".to_string());

    assert_eq!(tokens[0], Token::Char('a'));
    assert_eq!(tokens[1], Token::Char('b'));
    assert_eq!(tokens[2], Token::Operator(Operator::Concatenation));
    assert_eq!(tokens[3], Token::Char('c'));
    assert_eq!(tokens[4], Token::Operator(Operator::Concatenation));
    let nfa = construct_nfa(&tokens, 1);

    assert_eq!(nfa.final_states.len(), 1);
    assert!(nfa.transitions.len() >= 3);
}

#[test]
fn test_regex_simple_or_multiple_chars() {
    let tokens = regex_tokenizer(&"ab|cd".to_string());

    // ab -> a concat b, cd -> c concat d, puis | entre les deux
    let nfa = construct_nfa(&tokens, 1);

    assert!(nfa.final_states.len() >= 1);
    assert!(nfa.transitions.len() >= 3);
}

#[test]
fn test_regex_nested_or() {
    let tokens = regex_tokenizer(&"a|(b|c)".to_string());

    let nfa = construct_nfa(&tokens, 1);

    assert_eq!(tokens[0], Token::Char('a'));
    assert_eq!(tokens[1], Token::Char('b'));
    assert_eq!(tokens[2], Token::Char('c'));
    assert_eq!(tokens[3], Token::Operator(Operator::Or));
    assert_eq!(tokens[4], Token::Operator(Operator::Or));
    assert!(nfa.final_states.len() >= 1);
}

#[test]
fn test_regex_with_question_mark() {
    let tokens = regex_tokenizer(&"a?".to_string());

    let nfa = construct_nfa(&tokens, 1);

    assert_eq!(nfa.final_states.len(), 2);
}

#[test]
fn test_regex_plus_quantifier() {
    let tokens = regex_tokenizer(&"a+".to_string());

    let nfa = construct_nfa(&tokens, 1);

    // a+ = au moins une fois a
    assert_eq!(nfa.final_states.len(), 2);
    assert!(nfa.transitions.len() >= 2);
}

#[test]
fn test_regex_star_with_or() {
    let tokens = regex_tokenizer(&"(a|b)*".to_string());

    let nfa = construct_nfa(&tokens, 1);

    // boucle sur un choix entre a et b
    assert!(nfa.final_states.len() >= 1);
    assert!(nfa.transitions.len() >= 3);
}

#[test]
fn test_regex_char_class_equivalent() {
    let tokens = regex_tokenizer(&"[abc]".to_string());

    // Supposons que ton tokenizer transforme [abc] en a|b|c
    let nfa = construct_nfa(&tokens, 1);

    assert_eq!(nfa.final_states.len(), 3); // un pour chaque lettre
}

#[test]
fn test_regex_long_concat_with_kleene() {
    let tokens = regex_tokenizer(&"abc*".to_string());

    let nfa = construct_nfa(&tokens, 1);

    // a concat b concat c*
    assert!(nfa.final_states.len() >= 1);
    assert!(nfa.transitions.len() >= 4);
}
