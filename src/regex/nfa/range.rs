use std::collections::HashSet;

use crate::regex::{
    nfa::{
        concatenate::concatenate, offset::get_offset_from_nfa, repeat_exact::repeat_exact,
        utils::shift_states,
    },
    utils::VecUtils,
    NFA,
};

pub fn range(nfa: NFA, min: usize, max: usize) -> (NFA, usize) {
    assert!(min <= max, "Invalid range");
    assert!(max > 0, "Bad iteration values");

    if min == max {
        return repeat_exact(&nfa, min);
    }

    let mut result_nfa: Option<NFA> = None;
    let mut total_offset = 0;
    let mut accumulated_final_states = HashSet::new();

    let offset_increment = get_offset_from_nfa(&nfa);

    // Partie obligatoire (min répétitions)
    if min > 0 {
        let (mandatory_nfa, _) = repeat_exact(&nfa, min);
        total_offset = get_offset_from_nfa(&mandatory_nfa);
        accumulated_final_states = mandatory_nfa.final_states.clone();
        result_nfa = Some(mandatory_nfa);
    } else {
        accumulated_final_states.insert(0); // état initial final si min == 0
    }

    // Partie optionnelle (max - min répétitions)
    for _ in min..max {
        let shifted_nfa = shift_states(&nfa, &total_offset);

        // Ajout des nouveaux états finaux
		accumulated_final_states.extend(&shifted_nfa.final_states);

        total_offset += offset_increment;

        result_nfa = Some(match result_nfa {
            Some(current) => concatenate(current, shifted_nfa),
            None => shifted_nfa,
        });
    }

    // Mise à jour des états finaux
    let mut final_nfa = result_nfa.expect("Should have constructed a valid NFA");

	final_nfa.final_states.extend(accumulated_final_states);

    let next_id = total_offset + 1;
    return (final_nfa, next_id);
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
            final_states: HashSet::from([2]),
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
        assert_eq!(result_nfa.final_states, HashSet::from([4]));
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

        let (result_nfa, _) = range(nfa, min, max);


        // Vérifie que les états finaux et les transitions sont bien ajoutés
        assert_eq!(result_nfa.final_states.len(), 2); // 4 états finaux au total
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
    #[should_panic(expected = "Bad iteration values")]
    fn test_range_min_and_max_are_zero() {
        let nfa = create_test_nfa();
        let min = 0;
        let max = 0;

        let (result_nfa, _) = range(nfa, min, max);

        // Vérifie que l'automate résultant a seulement l'état initial comme final
        assert_eq!(result_nfa.final_states, HashSet::from([0]));
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

        let (result_nfa, _) = range(nfa, min, max);


        // Vérifie que les états finaux et les transitions sont bien ajoutés
        assert_eq!(result_nfa.final_states.len(), 4); // 4 états finaux au total
        assert_eq!(result_nfa.transitions.len(), 10); // 10 états en tout
    }
}
