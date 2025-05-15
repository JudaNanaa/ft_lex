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

    // États finaux avec double cercle
    dbg!(&dfa);
    for state in &dfa.new_final_states {
        writeln!(file, "  {} [shape=doublecircle];", state)?;
    }

    // États normaux (optionnel si tu veux forcer tous les états visibles)
    for state in dfa.new_transitions.keys() {
        if !dfa.new_final_states.contains(state) {
            writeln!(file, "  {} [shape=circle];", state)?;
        }
    }

    // Transitions
    for (from_state, transitions) in &dfa.new_transitions {
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

    // Appelle Graphviz pour générer le PNG
    Command::new("dot")
        .args(["-Tpng", "dfa.dot", "-o", "dfa.png"])
        .output()
        .expect("Échec lors de l'exécution de Graphviz (dot)");

    Ok(())
}
