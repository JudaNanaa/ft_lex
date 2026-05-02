use std::collections::{HashMap, HashSet, VecDeque};

#[cfg(feature = "dotfile")]
use crate::regex::dfa::dot::generate_dot_file;

use super::{Dfa, NewDfaTransition, State};
use crate::regex::Nfa;

fn find_target_state(nfa: &Nfa, current_state: &State, input_char: char) -> State {
    let mut target_states = HashSet::new();

    for nfa_state_id in &current_state.state {
        if let Some(transitions) = nfa.transitions.get(nfa_state_id) {
            for transition in transitions {
                if transition.input == input_char {
                    target_states.insert(transition.target_state);
                }
            }
        }
    }

    if target_states.is_empty() {
        return State::trap();
    }

    let mut sorted: Vec<usize> = target_states.into_iter().collect();
    sorted.sort_unstable();
    State { state: sorted }
}

pub fn build_dfa(nfa: &Nfa, eq_classes: &HashMap<char, usize>) -> Dfa {
    let mut dfa = Dfa::new();

    // Build one representative (lowest byte value char) per equivalence class
    let mut representatives: HashMap<usize, char> = HashMap::new();
    for byte in 0u8..=255 {
        let ch = char::from_u32(u32::from(byte)).unwrap();
        if let Some(&class_idx) = eq_classes.get(&ch) {
            representatives.entry(class_idx).or_insert(ch);
        }
    }
    // Convert to sorted Vec for deterministic iteration
    let mut repr_list: Vec<(usize, char)> = representatives.into_iter().collect();
    repr_list.sort_by_key(|&(idx, _)| idx);

    let mut state_map: HashMap<State, usize> = HashMap::new();
    let mut visited: HashSet<State> = HashSet::new();
    let mut pending: VecDeque<State> = VecDeque::new();
    let mut pending_set: HashSet<State> = HashSet::new();

    let initial = State { state: vec![0] };
    state_map.insert(initial.clone(), 0);
    dfa.nfa_states.insert(0, vec![0]);
    pending.push_back(initial.clone());
    pending_set.insert(initial);
    let mut next_id = 1usize;

    while let Some(current) = pending.pop_front() {
        pending_set.remove(&current);
        visited.insert(current.clone());
        let current_id = *state_map.get(&current).unwrap();

        if current
            .state
            .iter()
            .any(|s| nfa.trailing_states.contains(s))
        {
            dfa.trailing_states.insert(current_id);
        }

        if current
            .state
            .iter()
            .any(|s| nfa.trailing_final_states.contains(s))
        {
            dfa.trailing_final_states.insert(current_id);
        }

        let mut new_transitions = Vec::with_capacity(repr_list.len());

        for &(_class_idx, repr_char) in &repr_list {
            let target = find_target_state(nfa, &current, repr_char);
            if target.is_trap() {
                continue;
            }

            let target_id = if let Some(&id) = state_map.get(&target) {
                id
            } else {
                let id = next_id;
                dfa.nfa_states.insert(id, target.state.clone());
                state_map.insert(target.clone(), id);
                next_id += 1;
                if !visited.contains(&target) && !pending_set.contains(&target) {
                    pending_set.insert(target.clone());
                    pending.push_back(target.clone());
                }
                id
            };

            new_transitions.push(NewDfaTransition {
                input: repr_char,
                target_state: target_id,
            });
        }

        dfa.transitions.insert(current_id, new_transitions);
    }

    for (state, &id) in &state_map {
        if state.state.iter().any(|s| nfa.final_states.contains(s)) {
            dfa.final_states.insert(id);
        }
    }
    dfa.final_states.remove(&0);
    dfa.eq_classes.clone_from(eq_classes);

    #[cfg(feature = "dotfile")]
    if let Err(e) = generate_dot_file(&dfa) {
        eprintln!("Error generating dfa.dot: {e}");
    }

    dfa
}
