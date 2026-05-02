use crate::{
    file_parsing::{
        definitions::{parse::list_all_states, DefinitionState},
        FilePart,
    },
    lex_creation::SPACE,
};

pub fn write_yy_is_exclusive_state(
    file_parts: &FilePart,
    file: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    let definitions = file_parts.definitions();

    let all_condition_state = list_all_states(definitions);

    writeln!(file, "int yy_is_exclusive_state(int state) {{")?;
    writeln!(file, "{SPACE}switch (state) {{")?;

    for (state_name, state_type) in all_condition_state {
        if state_type == DefinitionState::Exclusive {
            writeln!(file, "{}case {}:", SPACE.repeat(2), state_name)?;
            writeln!(file, "{}return 1;", SPACE.repeat(3))?;
        }
    }
    writeln!(file, "{}default:", SPACE.repeat(2))?;
    writeln!(file, "{}return 0;", SPACE.repeat(3))?;
    writeln!(file, "{SPACE}}}")?;
    writeln!(file, "}}\n")?;
    Ok(())
}
