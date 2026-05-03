use crate::{file_parsing::FilePart, lex_creation::backend::CodegenBackend};

pub fn lex_creation(
    file_parts: &FilePart,
    backend: &dyn CodegenBackend,
    output: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    let mode = file_parts.yytext_mode;
    backend.write_header(file_parts.definitions(), mode, output)?;
    backend.write_yytext_section(mode, output)?;
    backend.write_tables(file_parts, output)?;
    backend.write_is_exclusive_state(file_parts, output)?;
    backend.write_action(file_parts, output)?;
    backend.write_accept_actions(file_parts, output)?;
    backend.write_yylex(file_parts.in_yylex(), mode, output)?;
    backend.write_user_routine(file_parts.user_routine(), output)?;
    Ok(())
}
