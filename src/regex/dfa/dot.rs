use std::process::Command;
use std::{fs::File, io::Write};

use super::DFA;

fn escape_label(label: &str) -> String {
    label
        .replace('\\', "\\\\") // échappe \ en \\
        .replace('"', "\\\"") // échappe " en \"
        .replace('\n', "\\\\n") // échappe retour ligne en \n
}

pub fn generate_dot_file(dfa: &DFA) -> std::io::Result<()> {
    let mut file = File::create("dfa.dot")?;

    writeln!(file, "digraph DFA {{")?;
    writeln!(file, "  rankdir=LR;")?;
    writeln!(file, "  node [shape=circle];")?;

    // États finaux avec double cercle
    for state in dfa.transitions.keys() {
        if dfa.final_states.contains(state) {
            writeln!(file, "  \"{:?}\" [shape=doublecircle];", state.state)?;
        }
    }

    // Transitions
    for (from_state, transitions) in &dfa.transitions {
        for transition in transitions {
            if transition.target_state.is_trap() {
                continue;
            }

            let escaped_label = escape_label(&transition.input.to_string());

            writeln!(
                file,
                "  \"{:?}\" -> \"{:?}\" [label=\"{}\"]",
                from_state.state, transition.target_state.state, escaped_label
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
