#[cfg(feature = "dotfile")]
use std::collections::HashMap;
#[cfg(feature = "dotfile")]
use std::process::Command;
#[cfg(feature = "dotfile")]
use std::{fs::File, io::Write};

#[cfg(feature = "dotfile")]
use super::Dfa;

#[cfg(feature = "dotfile")]
fn escape_char(c: char) -> String {
    match c {
        '"' => "\\\"".to_string(),
        '\\' => "\\\\".to_string(),
        '\n' => "\\\\n".to_string(),
        '\t' => "\\\\t".to_string(),
        '\r' => "\\\\r".to_string(),
        ' ' => "\\\\x20".to_string(),
        c if (c as u32) < 0x20 || c as u32 >= 0x7f => {
            format!("\\\\x{:02x}", c as u32)
        }
        c => c.to_string(),
    }
}

#[cfg(feature = "dotfile")]
fn is_dot_charset(sorted: &[char]) -> bool {
    // dot = all 255 bytes except '\n' (0x0a)
    if sorted.len() != 255 {
        return false;
    }
    sorted.iter().all(|&c| c != '\n')
        && (0u8..=255u8)
            .filter(|&b| b != b'\n')
            .zip(sorted.iter())
            .all(|(b, &c)| c as u32 == u32::from(b))
}

#[cfg(feature = "dotfile")]
fn build_edge_label(chars: &[char]) -> String {
    if chars.is_empty() {
        return String::new();
    }

    let mut sorted = chars.to_vec();
    sorted.sort_unstable();
    sorted.dedup();

    if is_dot_charset(&sorted) {
        return ".".to_string();
    }

    // Group into contiguous byte ranges
    let mut ranges: Vec<(char, char)> = Vec::new();
    let mut start = sorted[0];
    let mut end = sorted[0];

    for &c in sorted.iter().skip(1) {
        if c as u32 == end as u32 + 1 {
            end = c;
        } else {
            ranges.push((start, end));
            start = c;
            end = c;
        }
    }
    ranges.push((start, end));

    let parts: Vec<String> = ranges
        .iter()
        .map(|&(s, e)| {
            if s == e {
                escape_char(s)
            } else if e as u32 == s as u32 + 1 {
                format!("{}{}", escape_char(s), escape_char(e))
            } else {
                format!("{}-{}", escape_char(s), escape_char(e))
            }
        })
        .collect();

    if parts.len() == 1 && !parts[0].contains('-') {
        parts[0].clone()
    } else {
        format!("[{}]", parts.join(""))
    }
}

#[cfg(feature = "dotfile")]
pub fn generate_dot_file(dfa: &Dfa) -> std::io::Result<()> {
    let mut file = File::create("dfa.dot")?;

    // Build reverse eq_classes map: class_idx -> Vec<char>
    let mut class_to_chars: HashMap<usize, Vec<char>> = HashMap::new();
    for (&ch, &idx) in &dfa.eq_classes {
        class_to_chars.entry(idx).or_default().push(ch);
    }

    // Build a map: (from, to) -> Vec<char> by grouping transitions
    let mut edge_chars: HashMap<(usize, usize), Vec<char>> = HashMap::new();
    for (from_state, transitions) in &dfa.transitions {
        for transition in transitions {
            let chars = if let Some(&class_idx) = dfa.eq_classes.get(&transition.input) {
                class_to_chars
                    .get(&class_idx)
                    .cloned()
                    .unwrap_or_else(|| vec![transition.input])
            } else {
                vec![transition.input]
            };
            edge_chars
                .entry((*from_state, transition.target_state))
                .or_default()
                .extend(chars);
        }
    }

    writeln!(file, "digraph Dfa {{")?;
    writeln!(file, "  rankdir=LR;")?;
    writeln!(file, "  node [shape=circle];")?;

    // Invisible start node with arrow to initial state
    writeln!(file, "  __start [shape=none, label=\"\"];")?;
    writeln!(file, "  __start -> 0;")?;

    // State shapes: double circle for final, filled for trailing
    let mut all_states: Vec<usize> = dfa.transitions.keys().copied().collect();
    all_states.sort_unstable();

    for state in &all_states {
        let is_final = dfa.final_states.contains(state);
        let is_trailing = dfa.trailing_states.contains(state);
        let is_trailing_final = dfa.trailing_final_states.contains(state);

        let shape = if is_final || is_trailing_final {
            "doublecircle"
        } else {
            "circle"
        };
        let color = if is_trailing || is_trailing_final {
            " style=filled fillcolor=lightblue"
        } else {
            ""
        };
        writeln!(file, "  {} [shape={shape}{color}];", state)?;
    }

    // Emit one edge per (from, to) pair with grouped label
    let mut edges: Vec<((usize, usize), Vec<char>)> = edge_chars.into_iter().collect();
    edges.sort_by_key(|&((from, to), _)| (from, to));

    for ((from, to), chars) in edges {
        let label = build_edge_label(&chars);
        writeln!(file, "  {from} -> {to} [label=\"{label}\"];")?;
    }

    writeln!(file, "}}")?;
    drop(file);

    let result = Command::new("dot")
        .args(["-Tpng", "dfa.dot", "-o", "dfa.png"])
        .output();

    match result {
        Ok(out) if !out.status.success() => {
            eprintln!(
                "dot exited with error:\n{}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
        Err(e) => {
            eprintln!("Could not run graphviz dot: {e}");
        }
        _ => {}
    }

    Ok(())
}
