use crate::regex::{
    nfa::{
        concatenate::concatenate,
        repeat_exact::repeat_exact,
        utils::{pop_last_two, shift_states},
    },
    utils::VecUtils,
    NFA,
};

pub fn range(nfa: NFA, min: usize, max: usize) -> (NFA, usize) {
    assert!(min <= max, "Invalid range");
    assert!(max > 0, "bad iteration values");

    if min == max {
        return repeat_exact(&nfa, min);
    }

    let mut nfa_parts: Vec<NFA> = Vec::new();
    let mut total_offset = 0;
    let mut accumulated_final_states = Vec::new();

    // Partie obligatoire : min répétitions
    if min > 0 {
        let (mandatory_nfa, _) = repeat_exact(&nfa, min);
        accumulated_final_states = mandatory_nfa.final_states.clone();
        total_offset += mandatory_nfa.transitions.len();
        nfa_parts.push(mandatory_nfa);
    } else {
        accumulated_final_states.push(0); // L’état initial est final si min == 0
    }

    // Parties optionnelles : (max - min) répétitions
    for _ in min..max {
        let optional_nfa = shift_states(&nfa, total_offset);

        for state in &optional_nfa.final_states {
            accumulated_final_states.push_unique(*state);
        }

        total_offset += optional_nfa.transitions.len();
        nfa_parts.push(optional_nfa);

        if nfa_parts.len() == 2 {
            let (left, right) = pop_last_two(&mut nfa_parts);
            nfa_parts.push(concatenate(left, right));
        }
    }

    let mut final_nfa = nfa_parts.pop().unwrap();
    for state in accumulated_final_states {
        final_nfa.final_states.push_unique(state);
    }

    return (final_nfa, total_offset + 1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::regex::{Transition, NFA};
    use std::collections::HashMap;

    // Fonction pour créer un NFA simple
    fn create_test_nfa() -> NFA {
        let mut nfa = NFA {
            transitions: HashMap::new(),
            final_states: vec![2],
        };

        nfa.transitions.insert(
            0,
            vec![Transition {
                input: 'a',
                target_state: 1,
            }],
        );
        nfa.transitions.insert(
            1,
            vec![Transition {
                input: 'b',
                target_state: 2,
            }],
        );

        nfa
    }

    // Test de base de la fonction range avec min == max
    #[test]
    fn test_range_min_equals_max() {
        let nfa = create_test_nfa();
        let min = 2;
        let max = 2;

        let (result_nfa, _) = range(nfa, min, max);

        // Vérifie que l'automate a bien 2 répétitions
        assert_eq!(result_nfa.final_states, vec![4]);
        assert_eq!(result_nfa.transitions[&0].len(), 1);
        assert_eq!(result_nfa.transitions[&1].len(), 1);
        assert_eq!(result_nfa.transitions[&2].len(), 1);
        assert_eq!(result_nfa.transitions[&3].len(), 1);
    }

    // Test avec min > 0 et max > min
    #[test]

    fn test_range_min_less_than_max() {
        let nfa = create_test_nfa();
        let min = 2;
        let max = 3;

        let (mut result_nfa, _) = range(nfa, min, max);

        let expected_final_state = vec![4, 6];

        result_nfa.final_states.sort();
        // Vérifie que les états finaux et les transitions sont bien ajoutés
        assert_eq!(result_nfa.final_states.len(), 2); // 4 états finaux au total
        assert_eq!(result_nfa.final_states, expected_final_state);
        assert_eq!(result_nfa.transitions.len(), 6); // 10 états en tout
    }

    // Test avec min = 0 et max > 0
    #[test]
    fn test_range_min_is_zero() {
        let nfa = create_test_nfa();
        let min = 0;
        let max = 3;

        let (result_nfa, _) = range(nfa, min, max);

        // Vérifie que l'état initial est final lorsque min == 0
        assert!(result_nfa.final_states.contains(&0));
        assert_eq!(result_nfa.transitions[&0].len(), 1);
        assert_eq!(result_nfa.transitions[&1].len(), 1);
        assert_eq!(result_nfa.transitions[&2].len(), 1);
    }

    // Test avec min == 0 et max == 0
    #[test]
    #[should_panic(expected = "bad iteration values")]
    fn test_range_min_and_max_are_zero() {
        let nfa = create_test_nfa();
        let min = 0;
        let max = 0;

        let (result_nfa, _) = range(nfa, min, max);

        // Vérifie que l'automate résultant a seulement l'état initial comme final
        assert_eq!(result_nfa.final_states, vec![0]);
        assert_eq!(result_nfa.transitions[&0].len(), 1); // Une seule transition
    }

    // Test avec min > max (devrait échouer)
    #[test]
    #[should_panic(expected = "Invalid range")]
    fn test_range_invalid_range() {
        let nfa = create_test_nfa();
        let min = 3;
        let max = 2;

        range(nfa, min, max); // Doit panic
    }

    // Test avec une petite automaton répétée un nombre important de fois
    #[test]
    fn test_range_large_repeat() {
        let nfa = create_test_nfa();
        let min = 2;
        let max = 5;

        let (mut result_nfa, _) = range(nfa, min, max);

        let expected_final_state = vec![4, 6, 8, 10];

        result_nfa.final_states.sort();
        // Vérifie que les états finaux et les transitions sont bien ajoutés
        assert_eq!(result_nfa.final_states.len(), 4); // 4 états finaux au total
        assert_eq!(result_nfa.final_states, expected_final_state);
        assert_eq!(result_nfa.transitions.len(), 10); // 10 états en tout
    }
}
