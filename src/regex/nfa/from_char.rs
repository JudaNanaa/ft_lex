use std::collections::HashMap;

use super::{Transition, NFA};

pub fn from_char(c: char, state_id: &mut usize) -> NFA {
    let mut transitions: HashMap<usize, Vec<Transition>> = HashMap::new();
    let final_state = *state_id;

    let transition = Transition {
        input: c,
        target_state: final_state,
    };

    *state_id += 1;
    transitions.insert(0, vec![transition]);
    transitions.insert(final_state, vec![]);

    return NFA {
        transitions,
        final_states: vec![final_state],
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test de base pour la création d'un NFA avec un seul caractère
    #[test]
    fn test_from_char_basic() {
        let mut state_id = 1;
        let result = from_char('a', &mut state_id);

        // Vérifie que le NFA résultant a une transition correcte et un état final
        assert_eq!(result.transitions.len(), 2); // Il devrait y avoir une transition
        assert_eq!(result.transitions[&0].len(), 1); // L'état 0 a une seule transition
        assert_eq!(result.transitions[&0][0].input, 'a'); // La transition devrait utiliser 'a' comme caractère
        assert_eq!(result.transitions[&0][0].target_state, 1); // La transition doit mener à l'état final
        assert_eq!(result.final_states, vec![1]); // L'état final doit être 1
    }

    // Test avec un autre caractère
    #[test]
    fn test_from_char_different_char() {
        let mut state_id = 2;
        let result = from_char('b', &mut state_id);

        // Vérifie les mêmes éléments que le test précédent, mais avec un caractère différent
        assert_eq!(result.transitions.len(), 2);
        assert_eq!(result.transitions[&0].len(), 1);
        assert_eq!(result.transitions[&0][0].input, 'b');
        assert_eq!(result.transitions[&0][0].target_state, 2);
        assert_eq!(result.final_states, vec![2]);
    }

    // Test avec plusieurs appels pour vérifier la gestion du state_id
    #[test]
    fn test_from_char_multiple_calls() {
        let mut state_id = 1;

        // Appel de from_char deux fois
        let result1 = from_char('a', &mut state_id);
        let result2 = from_char('b', &mut state_id);

        // Vérification du premier NFA
        assert_eq!(result1.transitions.len(), 2);
        assert_eq!(result1.transitions[&0].len(), 1);
        assert_eq!(result1.transitions[&0][0].input, 'a');
        assert_eq!(result1.transitions[&0][0].target_state, 1);
        assert_eq!(result1.final_states, vec![1]);

        // Vérification du second NFA
        assert_eq!(result2.transitions.len(), 2);
        assert_eq!(result2.transitions[&0].len(), 1);
        assert_eq!(result2.transitions[&0][0].input, 'b');
        assert_eq!(result2.transitions[&0][0].target_state, 2);
        assert_eq!(result2.final_states, vec![2]);
    }

    // Test avec un état initial donné
    #[test]
    fn test_from_char_with_initial_state() {
        let mut state_id = 10;
        let result = from_char('c', &mut state_id);

        assert_eq!(result.transitions.len(), 2);
        assert_eq!(result.transitions[&0].len(), 1);
        assert_eq!(result.transitions[&0][0].input, 'c');
        assert_eq!(result.transitions[&0][0].target_state, 10);
        assert_eq!(result.final_states, vec![10]);
    }
}
