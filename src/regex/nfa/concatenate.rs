use super::Nfa;

pub fn concatenate(mut left: Nfa, mut right: Nfa) -> Nfa {
    let right_initial = right.transitions.remove(&0).unwrap_or_default();
    let right_has_initial_final = right.final_states.contains(&0);

    right.final_states.remove(&0);

    for &state in &left.final_states {
        left.transitions
            .entry(state)
            .or_default()
            .extend(right_initial.clone());
    }

    if right_has_initial_final {
        right.final_states.extend(left.final_states);
    }

    left.transitions.extend(right.transitions);

    Nfa {
        transitions: left.transitions,
        final_states: right.final_states,
        ..Nfa::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::regex::nfa::Transition;
    use std::collections::{HashMap, HashSet};

    // Fonction de création d'un Nfa de test simple
    fn create_test_nfa() -> Nfa {
        let mut nfa = Nfa {
            transitions: HashMap::new(),
            final_states: HashSet::from([2]),
            ..Nfa::new()
        };

        // Transition de 0 à 1 avec le caractère 'a'
        nfa.transitions.insert(
            0,
            vec![Transition {
                input: 'a',
                target_state: 1,
            }],
        );
        // Transition de 1 à 2 avec le caractère 'b'
        nfa.transitions.insert(
            1,
            vec![Transition {
                input: 'b',
                target_state: 2,
            }],
        );

        nfa
    }

    // Fonction de création d'un autre Nfa de test
    fn create_second_test_nfa() -> Nfa {
        let mut nfa = Nfa {
            transitions: HashMap::new(),
            final_states: HashSet::from([4]),
            ..Nfa::new()
        };

        // Transition de 0 à 1 avec le caractère 'c'
        nfa.transitions.insert(
            0,
            vec![Transition {
                input: 'c',
                target_state: 1,
            }],
        );
        // Transition de 1 à 4 avec le caractère 'd'
        nfa.transitions.insert(
            3,
            vec![Transition {
                input: 'd',
                target_state: 4,
            }],
        );

        nfa
    }

    // Test de base de la concaténation
    #[test]
    fn test_concatenate_basic() {
        let left = create_test_nfa();
        let right = create_second_test_nfa();

        let result = concatenate(left, right);

        // Vérifie les transitions du Nfa résultant
        assert_eq!(result.transitions[&0].len(), 1); // Il y a une transition partant de l'état 0
        assert_eq!(result.transitions[&0][0].input, 'a'); // La transition part de 0 avec 'a'
        assert_eq!(result.transitions[&1].len(), 1); // Il y a une transition partant de l'état 1
        assert_eq!(result.transitions[&1][0].input, 'b'); // La transition part de 1 avec 'b'
        assert_eq!(result.transitions[&2].len(), 1); // La transition part de 2 avec 'c' après la concaténation
        assert_eq!(result.transitions[&2][0].input, 'c'); // La transition part de 2 avec 'c'
        assert_eq!(result.transitions[&3].len(), 1); // La transition part de 3 avec 'd'
        assert_eq!(result.transitions[&3][0].input, 'd'); // La transition part de 3 avec 'd'

        // Vérifie les états finaux du Nfa résultant
        assert_eq!(result.final_states, HashSet::from([4]));
    }

    // Test de concaténation avec plusieurs états finaux dans le Nfa de gauche
    #[test]
    fn test_concatenate_with_multiple_final_states() {
        let left = create_test_nfa();
        let right = create_second_test_nfa();

        let result = concatenate(left, right);

        assert_eq!(result.transitions[&0].len(), 1);
        assert_eq!(result.transitions[&0][0].input, 'a');
        assert_eq!(result.transitions[&1].len(), 1);
        assert_eq!(result.transitions[&1][0].input, 'b');
        assert_eq!(result.transitions[&2].len(), 1);
        assert_eq!(result.transitions[&2][0].input, 'c');
        assert_eq!(result.transitions[&3].len(), 1);
        assert_eq!(result.transitions[&3][0].input, 'd');

        assert_eq!(result.final_states, HashSet::from([4]));
    }

    #[test]
    fn test_concatenate_without_initial_state() {
        let left = create_test_nfa();
        let mut right = Nfa {
            transitions: HashMap::new(),
            final_states: HashSet::from([3]),
            ..Nfa::new()
        };

        right.transitions.insert(
            0,
            vec![Transition {
                input: 'd',
                target_state: 3,
            }],
        );

        let result = concatenate(left, right);

        assert!(result.transitions.contains_key(&0));
        assert_eq!(result.transitions[&0].len(), 1);
        assert_eq!(result.transitions[&0][0].input, 'a');
        assert_eq!(result.transitions[&1].len(), 1);
        assert_eq!(result.transitions[&1][0].input, 'b');
        assert_eq!(result.transitions[&2].len(), 1);
        assert_eq!(result.transitions[&2][0].input, 'd');
        assert_eq!(result.final_states.len(), 1);
    }
}
