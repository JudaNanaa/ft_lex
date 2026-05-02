use super::{or::or, Nfa};

pub fn combine_nfa(rules: Vec<Nfa>) -> Nfa {
    let mut last_nfa: Option<Nfa> = None;

    for rule in rules {
        if let Some(left) = last_nfa {
            last_nfa = Some(or(left, rule));
        } else {
            last_nfa = Some(rule);
        }
    }

    last_nfa.expect("Not normal")
}
