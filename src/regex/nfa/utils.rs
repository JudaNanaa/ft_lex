use super::{Transition, NFA};

pub fn shift_states(nfa: &NFA, offset: &usize) -> NFA {
    let mut new_nfa = NFA::new();

    for (state, transitions) in &nfa.transitions {
        let new_key = if *state == 0 { 0 } else { state + offset };
        let updated_transitions: Vec<Transition> = transitions
            .clone()
            .into_iter()
            .map(|mut t| {
                t.target_state += offset;
                t
            })
            .collect();
        new_nfa.transitions.insert(new_key, updated_transitions);
    }

    new_nfa.final_states = nfa
        .final_states
        .clone()
        .iter()
        .map(|state| state + offset)
        .collect();

    return new_nfa;
}

pub fn pop_last_two(stack: &mut Vec<NFA>) -> (NFA, NFA) {
    let second = stack.pop().expect("Internal error");
    let first = stack.pop().expect("Internal error");
    return (first, second);
}

// ------------ Tests

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // Fonction de création d'un NFA simple
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

    // Test de la fonction shift_states
    #[test]
    fn test_shift_states() {
        let nfa = create_test_nfa();
        let offset = 2;

        let shifted_nfa = shift_states(&nfa, &offset);

        // Vérifie les transitions après décalage
        assert_eq!(shifted_nfa.transitions.len(), 2);
        assert!(shifted_nfa.transitions.contains_key(&3));

        // Vérifie les transitions des nouveaux états
        assert_eq!(shifted_nfa.transitions[&3][0].target_state, 4); // Transition de 3 à 4

        // Vérifie les états finaux après décalage
        assert_eq!(shifted_nfa.final_states, vec![4]);
    }

    // Test de la fonction shift_states avec un offset de 0 (aucun changement)
    #[test]
    fn test_shift_states_no_change() {
        let nfa = create_test_nfa();
        let offset = 0;

        let shifted_nfa = shift_states(&nfa, &offset);

        // Vérifie que l'automate n'a pas été modifié
        assert_eq!(shifted_nfa.transitions.len(), 2);
        assert_eq!(shifted_nfa.transitions[&0][0].target_state, 1);
        assert_eq!(shifted_nfa.transitions[&1][0].target_state, 2);

        // Vérifie que les états finaux sont identiques
        assert_eq!(shifted_nfa.final_states, vec![2]);
    }

    // Test de la fonction pop_last_two
    #[test]
    fn test_pop_last_two() {
        let nfa1 = create_test_nfa();
        let mut nfa2 = create_test_nfa();
        nfa2.final_states.push(3); // Ajoute un état final supplémentaire pour nfa2

        let mut stack: Vec<NFA> = Vec::new();
        stack.push(nfa1);
        stack.push(nfa2);

        let (first, second) = pop_last_two(&mut stack);

        // Vérifie que pop_last_two a correctement extrait les NFA
        assert_eq!(first.final_states, vec![2]);
        assert_eq!(second.final_states, vec![2, 3]);
    }

    // Test de la fonction pop_last_two avec une pile vide
    #[test]
    #[should_panic(expected = "Internal error")]
    fn test_pop_last_two_empty_stack() {
        let mut stack: Vec<NFA> = Vec::new();
        pop_last_two(&mut stack); // Devrait paniquer car la pile est vide
    }
}
