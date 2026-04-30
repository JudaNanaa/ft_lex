use crate::regex::{nfa::nfa::build_nfa, Operator, Quantifier, Token};

use super::regex_tokenizer;

#[test]
fn test_regex_1() {
    let tokens = regex_tokenizer("chef");

    assert_eq!(tokens[0], Token::Char('c'));
    assert_eq!(tokens[1], Token::Char('h'));
    assert_eq!(tokens[2], Token::Operator(Operator::Concatenation));
    assert_eq!(tokens[3], Token::Char('e'));
    assert_eq!(tokens[4], Token::Operator(Operator::Concatenation));
    assert_eq!(tokens[5], Token::Char('f'));
    assert_eq!(tokens[6], Token::Operator(Operator::Concatenation));

    let nfa = build_nfa(&tokens, &mut 1);

    assert_eq!(nfa.final_states.len(), 1);
}

#[test]
fn test_regex_2() {
    let tokens = regex_tokenizer("a|b");

    assert_eq!(tokens[0], Token::Char('a'));
    assert_eq!(tokens[1], Token::Char('b'));
    assert_eq!(tokens[2], Token::Operator(Operator::Or));

    let nfa = build_nfa(&tokens, &mut 1);

    assert_eq!(nfa.final_states.len(), 2);
}

#[test]
fn test_regex_3() {
    let tokens = regex_tokenizer("a*");

    assert_eq!(tokens[0], Token::Char('a'));
    assert_eq!(
        tokens[1],
        Token::Operator(Operator::Quantifier(Quantifier::AtLeast(0)))
    );

    let nfa = build_nfa(&tokens, &mut 1);

    assert_eq!(nfa.transitions.len(), 2);
    assert_eq!(nfa.final_states.len(), 2);
}

#[test]
fn test_regex_group_concatenation() {
    let tokens = regex_tokenizer("(ab)c");

    assert_eq!(tokens[0], Token::Char('a'));
    assert_eq!(tokens[1], Token::Char('b'));
    assert_eq!(tokens[2], Token::Operator(Operator::Concatenation));
    assert_eq!(tokens[3], Token::Char('c'));
    assert_eq!(tokens[4], Token::Operator(Operator::Concatenation));
    let nfa = build_nfa(&tokens, &mut 1);

    assert_eq!(nfa.final_states.len(), 1);
    assert!(nfa.transitions.len() >= 3);
}

#[test]
fn test_regex_simple_or_multiple_chars() {
    let tokens = regex_tokenizer("ab|cd");

    // ab -> a concat b, cd -> c concat d, puis | entre les deux
    let nfa = build_nfa(&tokens, &mut 1);

    assert!(!nfa.final_states.is_empty());
    assert!(nfa.transitions.len() >= 3);
}

#[test]
fn test_regex_nested_or() {
    let tokens = regex_tokenizer("a|(b|c)");

    let nfa = build_nfa(&tokens, &mut 1);

    assert_eq!(tokens[0], Token::Char('a'));
    assert_eq!(tokens[1], Token::Char('b'));
    assert_eq!(tokens[2], Token::Char('c'));
    assert_eq!(tokens[3], Token::Operator(Operator::Or));
    assert_eq!(tokens[4], Token::Operator(Operator::Or));
    assert!(!nfa.final_states.is_empty());
}

#[test]
fn test_regex_with_question_mark() {
    let tokens = regex_tokenizer("a?");

    let nfa = build_nfa(&tokens, &mut 1);

    assert_eq!(nfa.final_states.len(), 2);
}

#[test]
fn test_regex_plus_quantifier() {
    let tokens = regex_tokenizer("a+");

    let nfa = build_nfa(&tokens, &mut 1);

    // a+ = au moins une fois a
    assert_eq!(nfa.final_states.len(), 2);
    assert!(nfa.transitions.len() >= 2);
}

#[test]
fn test_regex_star_with_or() {
    let tokens = regex_tokenizer("(a|b)*");

    let nfa = build_nfa(&tokens, &mut 1);

    // boucle sur un choix entre a et b
    assert!(!nfa.final_states.is_empty());
    assert!(nfa.transitions.len() >= 3);
}

#[test]
fn test_regex_char_class_equivalent() {
    let tokens = regex_tokenizer("[abc]");

    // Supposons que ton tokenizer transforme [abc] en a|b|c
    let nfa = build_nfa(&tokens, &mut 1);

    assert_eq!(nfa.final_states.len(), 3); // un pour chaque lettre
}

#[test]
fn test_regex_long_concat_with_kleene() {
    let tokens = regex_tokenizer("abc*");

    let nfa = build_nfa(&tokens, &mut 1);

    // a concat b concat c*
    assert!(!nfa.final_states.is_empty());
    assert!(nfa.transitions.len() >= 4);
}

#[test]
fn test_regex_trailing_context_sets_trailing_states() {
    let tokens = regex_tokenizer("ab/cd");
    let nfa = build_nfa(&tokens, &mut 1);

    assert!(!nfa.trailing_states.is_empty());
    assert!(nfa.trailing_states.is_disjoint(&nfa.final_states));
}

#[test]
fn test_regex_trailing_context_or_preserves_trailing_states() {
    // Régression : combine_nfa utilise or() en boucle, les trailing_states
    // ne doivent pas être perdues après la fusion avec d'autres règles.
    use crate::regex::combine_nfa::combine_nfa;

    let tokens_trailing = regex_tokenizer("ab/cd");
    let nfa_trailing = build_nfa(&tokens_trailing, &mut 1);
    let expected = nfa_trailing.trailing_states.clone();

    let tokens_other = regex_tokenizer("xyz");
    let nfa_other = build_nfa(&tokens_other, &mut 10);

    let combined = combine_nfa(vec![nfa_trailing, nfa_other]);

    for state in expected {
        assert!(combined.trailing_states.contains(&state));
    }
}
