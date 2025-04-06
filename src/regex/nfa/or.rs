use super::NFA;

pub fn or(left: NFA, right: NFA) -> NFA {
    let mut transitions = left.transitions;
    let mut final_states = left.final_states;

    for (state, mut trans) in right.transitions {
        transitions
            .entry(state)
            .and_modify(|list| list.append(&mut trans))
            .or_insert(trans);
    }

    for state in right.final_states {
        if !final_states.contains(&state) {
            final_states.push(state);
        }
    }

    return NFA {
        transitions,
        final_states,
    };
}


// ------------------- Tests


#[cfg(test)]
mod tests {
    use crate::regex::nfa::Transition;

    use super::*;
    use std::collections::HashMap;

    // Fonction pour créer un NFA simple
    fn create_test_nfa_a() -> NFA {
        let mut nfa = NFA {
            transitions: HashMap::new(),
            final_states: vec![1],
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
            final_states: vec![2],
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
        assert_eq!(result.final_states, vec![1, 2]);
    }

}

