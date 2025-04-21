use super::{or::or, NFA};

pub fn combine_nfa(rules: Vec<NFA>) -> NFA {
    let mut last_nfa: Option<NFA> = None;

    for rule in rules {
        if let Some(left) = last_nfa {
            last_nfa = Some(or(left, rule));
        } else {
            last_nfa = Some(rule);
        }
    }

    let output = last_nfa.expect("Not normal");
    return output;
}
