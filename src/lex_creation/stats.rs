use crate::file_parsing::{definitions::Definition, FilePart};
use crate::lex_creation::tables::yy_nxt::{compute_yy_nxt, pack_yy_nxt};

pub struct Stats {
    pub rules: usize,
    pub nfa_states: usize,
    pub dfa_states: usize,
    pub eq_classes: usize,
    pub start_conditions: usize,
    pub total_table_entries: usize,
    pub packed_table_entries: Option<usize>,
}

pub fn compute_stats(file_parts: &FilePart, compressed: bool) -> Stats {
    let dfa_states = file_parts.dfa().transitions().len();
    let nfa_states = file_parts
        .dfa()
        .nfa_states
        .values()
        .flatten()
        .max()
        .copied()
        .unwrap_or(0)
        + 1;
    let eq_classes = file_parts
        .dfa()
        .eq_classes
        .values()
        .copied()
        .collect::<std::collections::HashSet<_>>()
        .len();
    let start_conditions = file_parts
        .definitions()
        .iter()
        .filter(|d| {
            matches!(
                d,
                Definition::InclusiveState { .. } | Definition::ExclusiveState { .. }
            )
        })
        .count()
        + 1;

    let yy_nxt_size = dfa_states * (eq_classes + 1);
    let yy_accept_size = dfa_states;
    let total_table_entries = yy_nxt_size + yy_accept_size + 256;

    let packed_table_entries = if compressed {
        let nxt = compute_yy_nxt(file_parts.dfa());
        let packed = pack_yy_nxt(&nxt);
        Some(packed.nxt.len())
    } else {
        None
    };

    Stats {
        rules: file_parts.rule_action().len(),
        nfa_states,
        dfa_states,
        eq_classes,
        start_conditions,
        total_table_entries,
        packed_table_entries,
    }
}

pub fn print_stats(stats: &Stats, output: &mut dyn std::io::Write) {
    let _ = writeln!(output, "ft_lex usage statistics:");
    let _ = writeln!(output, "  scanner options: -v");
    let _ = writeln!(output, "  {} NFA states", stats.nfa_states);
    let _ = writeln!(output, "  {} DFA states", stats.dfa_states);
    let _ = writeln!(output, "  {} rules", stats.rules);
    let _ = writeln!(output, "  {} start conditions", stats.start_conditions);
    let _ = writeln!(output, "  {} equivalence classes created", stats.eq_classes);
    match stats.packed_table_entries {
        None => {
            let _ = writeln!(
                output,
                "  {} total table entries needed",
                stats.total_table_entries
            );
        }
        Some(packed) => {
            let pct = 100usize.saturating_sub(packed * 100 / stats.total_table_entries.max(1));
            let _ = writeln!(
                output,
                "  {} table entries  ->  {} packed entries ({}% saved)",
                stats.total_table_entries, packed, pct
            );
        }
    }
}
