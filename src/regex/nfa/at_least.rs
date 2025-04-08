use super::{concatenate::concatenate, repeat_exact::repeat_exact, utils::shift_states, NFA};

fn apply_kleene_star(nfa: &mut NFA) {
    let initial_transitions = nfa
        .transitions
        .get(&0)
        .cloned()
        .expect("No initial state, internal error");

    for &final_state in &nfa.final_states {
        for initial_state in &initial_transitions {
            let tab = nfa.transitions.entry(final_state).or_default();
            if !tab.contains(initial_state) {
                tab.push(*initial_state);
            }
        }
    }

    if !nfa.final_states.contains(&0) {
        nfa.final_states.push(0);
    }
}

pub fn at_least(nfa: NFA, count: usize) -> (NFA, usize) {
    if count == 0 {
        let mut kleene = nfa.clone();
        apply_kleene_star(&mut kleene);
        let next_id = kleene.final_states.iter().max().unwrap() + 1;
        return (kleene, next_id);
    }

    let (repeated, _) = repeat_exact(&nfa, count);
    let mut kleene_part = nfa.clone();
    apply_kleene_star(&mut kleene_part);

    let shifted_kleene = shift_states(&kleene_part, &repeated.final_states.iter().max().unwrap());
    let result = concatenate(repeated, shifted_kleene);
    let next_id = result.final_states.iter().max().unwrap() + 1;
    return (result, next_id);
}

#[cfg(test)]
mod tests {
    use crate::regex::nfa::Transition;

    use super::*;
    use std::collections::HashMap;

    // Fonction de création d'un NFA de test simple
    fn create_test_nfa() -> NFA {
        let mut nfa = NFA {
            transitions: HashMap::new(),
            final_states: vec![2],
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

        return nfa;
    }

    // Test de l'application de Kleene Star
    #[test]
    fn test_apply_kleene_star() {
        let mut nfa = create_test_nfa();
        apply_kleene_star(&mut nfa);

        // Après application du Kleene Star
        assert_eq!(nfa.final_states, vec![2, 0]); // L'état initial 0 est aussi final
        assert!(nfa.transitions.contains_key(&2)); // La transition depuis l'état final vers l'état initial
        assert_eq!(
            nfa.transitions[&2],
            vec![Transition {
                input: 'a',
                target_state: 1
            },]
        );
    }

    // Test de la fonction at_least avec count = 0 (Kleene Star)
    #[test]
    fn test_at_least_zero() {
        let nfa = create_test_nfa();
        let (result_nfa, _) = at_least(nfa, 0);

        // Vérifie si l'automate résultant a bien l'état 0 comme état final avec une répétition de Kleene
        assert_eq!(result_nfa.final_states, vec![2, 0]);
        assert!(result_nfa.transitions.contains_key(&2));
        assert_eq!(
            result_nfa.transitions[&2],
            vec![Transition {
                input: 'a',
                target_state: 1
            },]
        );
    }

    // Test de la fonction at_least avec count > 0
    #[test]
    fn test_at_least_non_zero() {
        let nfa = create_test_nfa();
        let (result_nfa, next_id) = at_least(nfa, 3);

        // L'automate résultant devrait avoir appliqué le Kleene Star sur la répétition de 3 fois
        assert_eq!(result_nfa.final_states.len(), 2);
        assert!(result_nfa.final_states.contains(&(next_id - 1))); // Assumer que `states` soit disponible dans `NFA`
        assert!(result_nfa.transitions.contains_key(&(next_id - 1)));
    }

    // Test d'un NFA avec des transitions supplémentaires
    #[test]
    fn test_complex_nfa() {
        let mut nfa = NFA {
            transitions: HashMap::new(),
            final_states: vec![3],
        };

        // Transitions pour le NFA
        nfa.transitions.insert(
            0,
            vec![
                Transition {
                    input: 'a',
                    target_state: 1,
                },
                Transition {
                    input: 'b',
                    target_state: 2,
                },
            ],
        );
        nfa.transitions.insert(
            1,
            vec![Transition {
                input: 'a',
                target_state: 3,
            }],
        );
        nfa.transitions.insert(
            2,
            vec![Transition {
                input: 'b',
                target_state: 3,
            }],
        );

        apply_kleene_star(&mut nfa);

        // L'état initial 0 doit aussi être final
        assert!(nfa.final_states.contains(&0));
        assert_eq!(nfa.final_states, vec![3, 0]);

        // Tester les transitions
        assert!(nfa.transitions.contains_key(&3));
        assert!(nfa.transitions[&3]
            .iter()
            .any(|t| t.input == 'a' || t.input == 'b'));
    }

    // Test avec un NFA ayant plusieurs états
    #[test]
    fn test_multiple_states() {
        let mut nfa = NFA {
            transitions: HashMap::new(),
            final_states: vec![4],
        };

        // Définir plusieurs transitions
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
        nfa.transitions.insert(
            2,
            vec![Transition {
                input: 'c',
                target_state: 3,
            }],
        );
        nfa.transitions.insert(
            3,
            vec![Transition {
                input: 'd',
                target_state: 4,
            }],
        );

        apply_kleene_star(&mut nfa);

        // Vérifie que l'état final 4 a des transitions correctes
        assert!(nfa.transitions.contains_key(&4));
        assert_eq!(nfa.transitions[&4].len(), 1);
        assert_eq!(nfa.transitions[&4][0].input, 'a');
    }

    // Test de transition invalide
    #[test]
    fn test_invalid_transition() {
        let nfa = create_test_nfa();

        // Vérifie qu'il n'y a pas de transition définie pour 'c' à partir de l'état 0
        assert!(nfa
            .transitions
            .get(&0)
            .unwrap()
            .iter()
            .all(|t| t.input != 'c'));
    }

    // Test d'un NFA avec plusieurs répétitions
    #[test]
    fn test_multiple_repetitions() {
        let nfa = create_test_nfa();
        let (result_nfa, _) = at_least(nfa, 2);

        // Vérifie que l'automate contient le bon nombre d'états et transitions après les répétitions
        assert!(result_nfa.final_states.len() > 1);
        assert!(result_nfa.transitions.len() > 0);
    }
}
