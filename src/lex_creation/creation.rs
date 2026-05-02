use crate::{
    file_parsing::FilePart,
    lex_creation::{
        functions::{
            yy_action::yy_action, yy_final::yy_final,
            yy_is_exclusive_state::write_yy_is_exclusive_state,
            yy_search_final::create_yy_search_final,
        },
        tables::table::tables_creation,
        write::{
            write_defines, write_includes, write_user_routine, write_variables, write_yylex,
            write_yytext_section,
        },
    },
};

pub fn lex_creation(file_parts: &FilePart, output: &mut dyn std::io::Write) -> std::io::Result<()> {
    let mode = file_parts.yytext_mode;

    write_includes(output)?;
    write_defines(output, file_parts.definitions(), mode)?;
    write_variables(file_parts.definitions(), output)?;
    write_yytext_section(mode, output)?;

    tables_creation(file_parts, output)?;

    write_yy_is_exclusive_state(file_parts, output)?;

    yy_action(file_parts, output)?;

    yy_final(file_parts, output)?;
    create_yy_search_final(file_parts.actions(), output)?;

    write_yylex(output, file_parts.in_yylex(), mode)?;

    write_user_routine(file_parts.user_routine(), output)?;

    Ok(())
}
