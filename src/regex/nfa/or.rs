use super::NFA;

pub fn or(left: NFA, right: NFA) -> NFA {
    let mut transitions = left.transitions;
    let mut final_states = left.final_states;
    let mut trailing_states = left.trailing_states;
    let mut trailing_final_states = left.trailing_final_states;

    for (state, mut trans) in right.transitions {
        transitions
            .entry(state)
            .and_modify(|list| list.append(&mut trans))
            .or_insert(trans);
    }

    final_states.extend(right.final_states);
    trailing_states.extend(right.trailing_states);
    trailing_final_states.extend(right.trailing_final_states);

    NFA {
        transitions,
        final_states,
        trailing_states,
        trailing_final_states,
    }
}

// ------------------- Tests

#[cfg(test)]
mod tests {
    use crate::regex::nfa::Transition;

    use super::*;
    use std::collections::{HashMap, HashSet};

    // Fonction pour créer un NFA simple
    fn create_test_nfa_a() -> NFA {
        let mut nfa = NFA {
            transitions: HashMap::new(),
            final_states: HashSet::from([1]),
            ..NFA::new()
        };

        nfa.transitions.insert(
            0,
            vec![Transition {
                input: 'a',
                target_state: 1,
            }],
        );

        nfa
    }

    // Fonction pour créer un autre NFA simple
    fn create_test_nfa_b() -> NFA {
        let mut nfa = NFA {
            transitions: HashMap::new(),
            final_states: HashSet::from([2]),
            ..NFA::new()
        };

        nfa.transitions.insert(
            0,
            vec![Transition {
                input: 'b',
                target_state: 2,
            }],
        );

        nfa
    }

    // Test de la fonction or pour vérifier l'union de deux NFAs
    #[test]
    fn test_or_basic() {
        let nfa_a = create_test_nfa_a();
        let nfa_b = create_test_nfa_b();

        let result = or(nfa_a, nfa_b);

        let expected_transition: Vec<Transition> = vec![
            Transition {
                input: 'a',
                target_state: 1,
            },
            Transition {
                input: 'b',
                target_state: 2,
            },
        ];

        // Vérification des transitions
        assert_eq!(result.transitions.len(), 1); // Il y a 2 états (0 et 1 ou 2)
        assert_eq!(result.final_states.len(), 2); // Il y a 2 états finaux (1 ou 2)
        assert_eq!(result.transitions[&0].len(), 2); // 0 a 2 transitions
        assert!(result.transitions[&0].contains(&expected_transition[0]));
        assert!(result.transitions[&0].contains(&expected_transition[1]));

        // Vérification des états finaux
        assert_eq!(result.final_states, HashSet::from([1, 2]));
    }

    #[test]
    fn test_or_trailing_states_empty_when_none() {
        let result = or(create_test_nfa_a(), create_test_nfa_b());
        assert!(result.trailing_states.is_empty());
    }

    #[test]
    fn test_or_preserves_trailing_states_from_left() {
        let mut nfa_a = create_test_nfa_a();
        nfa_a.trailing_states = HashSet::from([1]);
        let nfa_b = create_test_nfa_b();

        let result = or(nfa_a, nfa_b);
        assert_eq!(result.trailing_states, HashSet::from([1]));
    }

    #[test]
    fn test_or_merges_trailing_states_from_both() {
        let mut nfa_a = create_test_nfa_a();
        nfa_a.trailing_states = HashSet::from([1]);
        let mut nfa_b = create_test_nfa_b();
        nfa_b.trailing_states = HashSet::from([2]);

        let result = or(nfa_a, nfa_b);
        assert_eq!(result.trailing_states, HashSet::from([1, 2]));
    }

    #[test]
    fn test_or_preserves_trailing_final_states_from_left() {
        let mut nfa_a = create_test_nfa_a();
        nfa_a.trailing_final_states = HashSet::from([1]);
        let nfa_b = create_test_nfa_b();

        let result = or(nfa_a, nfa_b);
        assert_eq!(result.trailing_final_states, HashSet::from([1]));
    }

    #[test]
    fn test_or_merges_trailing_final_states_from_both() {
        let mut nfa_a = create_test_nfa_a();
        nfa_a.trailing_final_states = HashSet::from([1]);
        let mut nfa_b = create_test_nfa_b();
        nfa_b.trailing_final_states = HashSet::from([2]);

        let result = or(nfa_a, nfa_b);
        assert_eq!(result.trailing_final_states, HashSet::from([1, 2]));
    }

    #[test]
    fn test_or_trailing_final_states_empty_when_none() {
        let result = or(create_test_nfa_a(), create_test_nfa_b());
        assert!(result.trailing_final_states.is_empty());
    }
}
