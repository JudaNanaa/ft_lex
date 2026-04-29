#[cfg(feature = "dotfile")]
use std::process::Command;
#[cfg(feature = "dotfile")]
use std::{fs::File, io::Write};

#[cfg(feature = "dotfile")]
use super::DFA;

#[cfg(feature = "dotfile")]
fn escape_label(label: &str) -> String {
    label
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\\\n")
}

#[cfg(feature = "dotfile")]
pub fn generate_dot_file(dfa: &DFA) -> std::io::Result<()> {
    let mut file = File::create("dfa.dot")?;

    writeln!(file, "digraph DFA {{")?;
    writeln!(file, "  rankdir=LR;")?;
    writeln!(file, "  node [shape=circle];")?;

    for state in &dfa.final_states {
        writeln!(file, "  {} [shape=doublecircle];", state)?;
    }

    for state in dfa.transitions.keys() {
        if !dfa.final_states.contains(state) {
            writeln!(file, "  {} [shape=circle];", state)?;
        }
    }

    for (from_state, transitions) in &dfa.transitions {
        for transition in transitions {
            let escaped_label = escape_label(&transition.input.to_string());
            writeln!(
                file,
                "  {} -> {} [label=\"{}\"]",
                from_state, transition.target_state, escaped_label
            )?;
        }
    }

    writeln!(file, "}}")?;

    Command::new("dot")
        .args(["-Tpng", "dfa.dot", "-o", "dfa.png"])
        .output()
        .expect("Échec lors de l'exécution de Graphviz (dot)");

    Ok(())
}
