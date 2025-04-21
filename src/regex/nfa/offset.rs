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
