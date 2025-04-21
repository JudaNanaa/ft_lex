use super::NFA;

pub fn get_offset_from_nfa(nfa: &NFA) -> usize {
    return *nfa.final_states.iter().max().unwrap() + 1
        - nfa
            .transitions
            .keys()
            .copied()
            .filter(|&s| s != 0)
            .min()
            .unwrap();
}

#[cfg(test)]
mod tests {
    use crate::regex::Transition;

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_new_nfa() {
        let nfa = NFA::new();
        assert_eq!(nfa.transitions.len(), 0);
        assert_eq!(nfa.final_states.len(), 0);
    }

    #[test]
    fn test_get_offset_simple() {
        let mut nfa = NFA::new();
        
        // Ajouter des transitions
        nfa.transitions.insert(1, vec![
            Transition { input: 'a', target_state: 2 },
        ]);
        nfa.transitions.insert(2, vec![
            Transition { input: 'b', target_state: 3 },
        ]);
        
        // Définir les états finaux
        nfa.final_states = vec![3];
        
        // Le max des états finaux est 3, +1 = 4
        // Le min des états non-zéro est 1
        // Donc le décalage devrait être 4 - 1 = 3
        assert_eq!(get_offset_from_nfa(&nfa), 3);
    }

    #[test]
    fn test_get_offset_multiple_final_states() {
        let mut nfa = NFA::new();
        
        nfa.transitions.insert(1, vec![
            Transition { input: 'a', target_state: 2 },
        ]);
        nfa.transitions.insert(2, vec![
            Transition { input: 'b', target_state: 3 },
            Transition { input: 'c', target_state: 4 },
        ]);
        
        nfa.final_states = vec![3, 4, 5];
        
        // Le max des états finaux est 5, +1 = 6
        // Le min des états non-zéro est 1
        // Donc le décalage devrait être 6 - 1 = 5
        assert_eq!(get_offset_from_nfa(&nfa), 5);
    }

    #[test]
    fn test_get_offset_with_state_zero() {
        let mut nfa = NFA::new();
        
        nfa.transitions.insert(0, vec![
            Transition { input: 'a', target_state: 2 },
        ]);
        nfa.transitions.insert(2, vec![
            Transition { input: 'b', target_state: 3 },
        ]);
        
        nfa.final_states = vec![3];
        
        // Le max des états finaux est 3, +1 = 4
        // Le min des états non-zéro est 2 (l'état 0 est ignoré dans le calcul)
        // Donc le décalage devrait être 4 - 2 = 2
        assert_eq!(get_offset_from_nfa(&nfa), 2);
    }

    #[test]
    fn test_get_offset_with_larger_state_numbers() {
        let mut nfa = NFA::new();
        
        nfa.transitions.insert(5, vec![
            Transition { input: 'a', target_state: 10 },
        ]);
        nfa.transitions.insert(10, vec![
            Transition { input: 'b', target_state: 15 },
        ]);
        
        nfa.final_states = vec![15, 20];
        
        // Le max des états finaux est 20, +1 = 21
        // Le min des états non-zéro est 5
        // Donc le décalage devrait être 21 - 5 = 16
        assert_eq!(get_offset_from_nfa(&nfa), 16);
    }

    #[test]
    fn test_get_offset_with_non_sequential_states() {
        let mut nfa = NFA::new();
        
        nfa.transitions.insert(3, vec![
            Transition { input: 'a', target_state: 7 },
        ]);
        nfa.transitions.insert(7, vec![
            Transition { input: 'b', target_state: 12 },
        ]);
        
        nfa.final_states = vec![12];
        
        // Le max des états finaux est 12, +1 = 13
        // Le min des états non-zéro est 3
        // Donc le décalage devrait être 13 - 3 = 10
        assert_eq!(get_offset_from_nfa(&nfa), 10);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_get_offset_empty_nfa_should_panic() {
        let nfa = NFA::new();
        get_offset_from_nfa(&nfa); // Devrait paniquer car pas d'états finaux
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_get_offset_only_zero_state_should_panic() {
        let mut nfa = NFA::new();
        
        nfa.transitions.insert(0, vec![
            Transition { input: 'a', target_state: 0 },
        ]);
        nfa.final_states = vec![0];
        
        get_offset_from_nfa(&nfa); // Devrait paniquer car pas d'états non-zéro
    }
}