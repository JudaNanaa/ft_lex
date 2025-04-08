use crate::regex::{
    nfa::{
        at_least::at_least, concatenate::concatenate, from_char::from_char, or::or, range::range,
        repeat_exact::repeat_exact, utils::pop_last_two,
    },
    Operator, Quantifier, Token, NFA,
};

pub fn construct_nfa(tokens: &Vec<Token>) -> NFA {
    let mut stack: Vec<NFA> = Vec::new();
    let mut state_id = 1;

    for token in tokens {
        let nfa = match *token {
            Token::Char(c) => from_char(c, &mut state_id),
            Token::Operator(op) => match op {
                Operator::Quantifier(q) => match q {
                    Quantifier::AtLeast(n) => {
                        let base = stack.pop().expect("Error applying Kleene star");
                        let (new_nfa, new_id) = at_least(base, n);
                        state_id = new_id;
                        new_nfa
                    }
                    Quantifier::Equal(n) => {
                        let base = stack.pop().expect("Error applying Equal");
                        let (new_nfa, new_id) = repeat_exact(&base, n);
                        state_id = new_id;
                        new_nfa
                    }
                    Quantifier::Range(min, max) => {
                        let base = stack.pop().expect("Error applying Range");
                        let (new_nfa, new_id) = range(base, min, max);
                        state_id = new_id;
                        new_nfa
                    }
                },
                Operator::Concatenation | Operator::TrailingContent => {
                    let (left, right) = pop_last_two(&mut stack);
                    concatenate(left, right)
                }
                Operator::Or => {
                    let (left, right) = pop_last_two(&mut stack);
                    or(left, right)
                }
                _ => panic!("Internal error"),
            },
        };
        stack.push(nfa);
    }
    let mut output = stack.pop().unwrap();
    output.final_states.sort();
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::regex::{Operator, Quantifier, Token};

    // Test pour la construction d'un NFA simple avec un caractère
    #[test]
    fn test_construct_nfa_single_char() {
        let tokens = vec![Token::Char('a')];
        let result = construct_nfa(&tokens);

        // Vérifie que le résultat est un NFA valide (en fonction de l'implémentation de `from_char`)
        assert_eq!(result.final_states, vec![1]);
    }

    // Test pour la concaténation de deux caractères
    #[test]
    fn test_construct_nfa_concatenation() {
        let tokens = vec![
            Token::Char('a'),
            Token::Char('b'),
            Token::Operator(Operator::Concatenation),
        ];
        let result = construct_nfa(&tokens);

        // Vérifie la concaténation des deux caractères 'a' et 'b'
        assert_eq!(result.final_states, vec![2]);
    }

    // Test pour l'opérateur OR entre deux caractères
    #[test]
    fn test_construct_nfa_or() {
        let tokens = vec![
            Token::Char('a'),
            Token::Char('b'),
            Token::Operator(Operator::Or),
        ];
        let result = construct_nfa(&tokens);

        // Vérifie que l'OR entre 'a' et 'b' a été correctement appliqué
        assert_eq!(result.final_states, vec![1, 2]);
    }

    // Test pour l'opérateur de répétition exacte
    #[test]
    fn test_construct_nfa_repeat_exact() {
        let tokens = vec![
            Token::Char('a'),
            Token::Operator(Operator::Quantifier(Quantifier::Equal(3))),
        ];
        let result = construct_nfa(&tokens);

        // Vérifie que la répétition exacte de 3 fois de 'a' a été appliquée
        assert_eq!(result.final_states, vec![3]);
    }

    // Test pour l'opérateur "at least" avec un minimum de répétitions
    #[test]
    fn test_construct_nfa_at_least() {
        let tokens = vec![
            Token::Char('a'),
            Token::Operator(Operator::Quantifier(Quantifier::AtLeast(2))),
        ];
        let result = construct_nfa(&tokens);

        // Vérifie que la répétition "at least" a été correctement appliquée
        assert_eq!(result.transitions.len(), 4); // 4 états au total
        assert_eq!(result.final_states, vec![2, 3]);
    }

    // Test pour l'opérateur de plage de répétitions (min, max)
    #[test]
    fn test_construct_nfa_range() {
        let tokens = vec![
            Token::Char('a'),
            Token::Operator(Operator::Quantifier(Quantifier::Range(2, 4))),
        ];
        let result = construct_nfa(&tokens);

        // Vérifie que la plage de répétition a été correctement appliquée
        assert_eq!(result.final_states, vec![2, 3, 4]);
    }

    // Test combiné avec plusieurs opérateurs (Concaténation et OR)
    #[test]
    fn test_construct_nfa_combined_operators() {
        let tokens = vec![
            Token::Char('a'),
            Token::Char('b'),
            Token::Operator(Operator::Concatenation),
            Token::Char('c'),
            Token::Operator(Operator::Or),
        ];
        let result = construct_nfa(&tokens);

        // Vérifie que la concaténation a bien eu lieu avant l'OR
        assert_eq!(result.final_states, vec![2, 3]);
    }

    // Fonction utilitaire pour vérifier les transitions d'un NFA
    fn check_transition(nfa: &NFA, state: usize, input: char, target_state: usize) {
        if let Some(transitions) = nfa.transitions.get(&state) {
            assert!(
                transitions
                    .iter()
                    .any(|t| t.input == input && t.target_state == target_state),
                "Transition de {} avec '{}' vers {} non trouvée",
                state,
                input,
                target_state
            );
        } else {
            panic!("Aucune transition trouvée pour l'état {}", state);
        }
    }

    // Test pour la concaténation de plusieurs caractères et l'OR entre deux parties
    #[test]
    fn test_construct_nfa_concat_and_or() {
        let tokens = vec![
            Token::Char('a'),
            Token::Char('b'),
            Token::Operator(Operator::Concatenation),
            Token::Char('c'),
            Token::Operator(Operator::Or),
        ];
        let result = construct_nfa(&tokens);

        // Vérifie que le NFA contient les bonnes transitions pour a+b|c
        check_transition(&result, 0, 'a', 1);
        check_transition(&result, 1, 'b', 2);
        check_transition(&result, 0, 'c', 3);

        assert_eq!(result.final_states.len(), 2); // 2 états finaux (2 et 3)
    }

    // Test pour des répétitions exactes avec plusieurs éléments dans le token
    #[test]
    fn test_construct_nfa_repeat_exact_complex() {
        let tokens = vec![
            Token::Char('a'),
            Token::Operator(Operator::Quantifier(Quantifier::Equal(2))),
            Token::Char('b'),
            Token::Operator(Operator::Quantifier(Quantifier::Equal(3))),
            Token::Operator(Operator::Concatenation),
        ];
        let result = construct_nfa(&tokens);

        // Vérifie que le NFA est construit avec 2 répétitions exactes de 'a' et 3 de 'b'
        assert_eq!(result.final_states.len(), 1); // 1 état final
    }

    // Test pour la répétition "at least" avec plusieurs répétitions
    #[test]
    fn test_construct_nfa_at_least_complex() {
        let tokens = vec![
            Token::Char('a'),
            Token::Operator(Operator::Quantifier(Quantifier::AtLeast(2))),
            Token::Char('b'),
            Token::Operator(Operator::Quantifier(Quantifier::AtLeast(1))),
            Token::Operator(Operator::Concatenation),
        ];
        let result = construct_nfa(&tokens);

        // Vérifie que le NFA est construit avec au moins 2 'a' et 1 'b'
        assert_eq!(result.transitions.len(), 6); // 6 états au total
        assert_eq!(result.final_states.len(), 2); // 2 états finaux
    }

    // Test pour l'opérateur "range" avec une plage de répétitions variable
    #[test]
    fn test_construct_nfa_range_complex() {
        let tokens = vec![
            Token::Char('a'),
            Token::Operator(Operator::Quantifier(Quantifier::Range(2, 4))),
            Token::Char('b'),
            Token::Operator(Operator::Quantifier(Quantifier::Range(1, 3))),
            Token::Operator(Operator::Concatenation),
        ];
        let result = construct_nfa(&tokens);

        // Vérifie que le NFA est correctement construit pour 'a{2,4}' et 'b{1,3}'
        assert_eq!(result.final_states.len(), 3); // 3 états finaux
    }

    // Test pour des combinaisons complexes avec concaténation, OR et quantificateurs
    #[test]
    fn test_construct_nfa_combined_complex() {
        let tokens = vec![
            Token::Char('a'),
            Token::Operator(Operator::Quantifier(Quantifier::AtLeast(1))),
            Token::Char('b'),
            Token::Operator(Operator::Quantifier(Quantifier::Equal(2))),
            Token::Operator(Operator::Concatenation),
            Token::Char('c'),
            Token::Operator(Operator::Concatenation),
            Token::Char('d'),
            Token::Operator(Operator::Or),
        ];
        let result = construct_nfa(&tokens);

        assert_eq!(result.final_states.len(), 2); // 2 états finaux
    }

    // Test pour des expressions vides et erreurs avec des quantificateurs invalides
    #[test]
    #[should_panic(expected = "Error applying Kleene star")]
    fn test_construct_nfa_empty_expression() {
        let tokens = vec![
            Token::Operator(Operator::Quantifier(Quantifier::AtLeast(0))), // Quantificateur invalide
        ];
        construct_nfa(&tokens);
    }

    // Test pour des expressions contenant seulement des opérateurs et vérification de la gestion des erreurs
    #[test]
    #[should_panic(expected = "Internal error")]
    fn test_construct_nfa_invalid_operator_sequence() {
        let tokens = vec![Token::Operator(Operator::Concatenation), Token::Char('a')];
        let result = construct_nfa(&tokens);

        // Le résultat devrait avoir un seul état avec une transition vers un état final
        assert_eq!(result.transitions.len(), 1);
        assert_eq!(result.final_states.len(), 1);
    }

    // Test pour la gestion de plusieurs états après application de OR et concaténation
    #[test]
    fn test_construct_nfa_or_and_concat() {
        let tokens = vec![
            Token::Char('a'),
            Token::Char('b'),
            Token::Operator(Operator::Or),
            Token::Char('c'),
            Token::Operator(Operator::Concatenation),
        ];
        let result = construct_nfa(&tokens);

        // Vérifie les transitions entre a|b et c
        check_transition(&result, 0, 'a', 1);
        check_transition(&result, 0, 'b', 2);
        check_transition(&result, 1, 'c', 3);
        check_transition(&result, 2, 'c', 3);

        assert_eq!(result.final_states, vec![3]); // 3 est l'état final
    }

    // Test pour un très grand NFA (pour tester les performances)
    #[test]
    fn test_construct_nfa_large_input() {
        let tokens: Vec<Token> = vec![
            Token::Char('a'),
            Token::Operator(Operator::Quantifier(Quantifier::AtLeast(1000))),
        ];

        let result = construct_nfa(&tokens);

        // Teste si le NFA est bien construit avec 1000+ répétitions de 'a'
        assert!(result.transitions.len() >= 1000); // +1000 transitions
        assert_eq!(result.final_states.len(), 2);
    }
}
