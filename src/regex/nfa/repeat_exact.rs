use crate::NFA;

use super::{concatenate::concatenate, utils::shift_states};

pub fn repeat_exact(nfa: &NFA, count: usize) -> (NFA, usize) {
    let mut big_nfa: Option<NFA> = None;
    let mut offset = 0;
    let max_first_final_state = nfa.final_states.iter().max().unwrap();

    if count == 0 {
        panic!("iteration value must be positive");
    }
    for _ in 0..count {
        let shifted = shift_states(nfa, &offset);
        offset += max_first_final_state;
        if let Some(left) = big_nfa {
			big_nfa = Some(concatenate(left, shifted));
        } else {
            big_nfa = Some(shifted);
        }
    }

    let output = big_nfa.unwrap();
    let next_id = output.final_states.iter().max().unwrap() + 1;
    return (output, next_id);
}

// ------------- Tests

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

    // Test de la fonction repeat_exact avec count == 1
    #[test]
    fn test_repeat_exact_count_1() {
        let nfa = create_test_nfa();
        let count = 1;

        let (result_nfa, _) = repeat_exact(&nfa, count);

        // Vérifie que l'automate résultant a bien les transitions et les états
        assert_eq!(result_nfa.final_states, vec![2]);
        assert_eq!(result_nfa.transitions[&0].len(), 1);
        assert_eq!(result_nfa.transitions[&1].len(), 1);
        assert_eq!(result_nfa, nfa);
    }

    // Test de la fonction repeat_exact avec count > 1
    #[test]
    fn test_repeat_exact_count_2() {
        let nfa = create_test_nfa();
        let count = 2;

        let (result_nfa, _) = repeat_exact(&nfa, count);

        // Vérifie que l'automate résultant a bien les états et transitions pour les 2 répétitions
        assert_eq!(result_nfa.final_states.len(), 1); // Deux états finaux à cause de la répétition
        assert_eq!(result_nfa.transitions[&0].len(), 1);
        assert_eq!(result_nfa.transitions[&1].len(), 1);
        assert_eq!(result_nfa.transitions[&2].len(), 1); // Transition supplémentaire
    }

    // Test de la fonction repeat_exact avec count > 2
    #[test]
    fn test_repeat_exact_count_3() {
        let nfa = create_test_nfa();
        let count = 3;

        let (result_nfa, _) = repeat_exact(&nfa, count);

        // Vérifie que l'automate résultant a bien les états et transitions pour les 3 répétitions
        assert_eq!(result_nfa.final_states.len(), 1); // Trois états finaux
        assert_eq!(result_nfa.transitions[&0].len(), 1);
        assert_eq!(result_nfa.transitions[&1].len(), 1);
        assert_eq!(result_nfa.transitions[&2].len(), 1);
        assert_eq!(result_nfa.transitions[&3].len(), 1); // Transition supplémentaire
    }

    // Test de la fonction repeat_exact avec count == 0 (devrait échouer)
    #[test]
    #[should_panic(expected = "iteration value must be positive")]
    fn test_repeat_exact_count_0() {
        let nfa = create_test_nfa();
        let count = 0;

        repeat_exact(&nfa, count); // Doit panic
    }

}
