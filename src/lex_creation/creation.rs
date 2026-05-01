use std::fs::File;

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
        LEX_FILE,
    },
};

pub fn lex_creation(file_parts: &FilePart) -> std::io::Result<()> {
    let mode = file_parts.yytext_mode;

    let mut file = File::create(LEX_FILE)?;

    write_includes(&mut file)?;
    write_defines(&mut file, file_parts.definitions(), mode)?;
    write_variables(file_parts.definitions(), &mut file)?;
    write_yytext_section(mode, &mut file)?;

    tables_creation(file_parts, &mut file)?;

    write_yy_is_exclusive_state(file_parts, &mut file)?;

    yy_action(file_parts, &mut file)?;

    yy_final(file_parts, &mut file)?;
    create_yy_search_final(file_parts.actions(), &mut file)?;

    write_yylex(&mut file, file_parts.in_yylex(), mode)?;

    write_user_routine(file_parts.user_routine(), &mut file)?;

    Ok(())
}
