use std::fs::File;

use crate::{
    file_parsing::FilePart,
    lex_creation::{
        functions::{
            yy_action::yy_action, yy_final::yy_final, yy_search_final::create_yy_search_final,
        },
        tables::table::tables_creation,
        write::{
            write_defines, write_includes, write_user_routine, write_variables,
            write_yy_add_buffer, write_yy_fatal_error, write_yy_if_match, write_yy_if_no_match,
            write_yy_increase_accepting_stack_len, write_yy_init_accepting_stack,
            write_yy_init_buffer, write_yy_next_char, write_yy_pop_accepting_state,
            write_yy_push_accepting_state, write_yy_reject, write_yy_set_yytext, write_yylex,
        },
        LEX_FILE,
    },
};

use super::write::write_yymore;

pub fn lex_creation(file_parts: FilePart) -> std::io::Result<()> {
    let mut file = File::create(LEX_FILE)?;

    write_includes(&mut file)?;
    write_defines(&mut file)?;
    write_variables(file_parts.definitions(), &mut file)?;

    tables_creation(&file_parts, &mut file)?;

    write_yy_fatal_error(&mut file)?;
    write_yy_init_buffer(&mut file)?;
    write_yy_init_accepting_stack(&mut file)?;
    write_yy_increase_accepting_stack_len(&mut file)?;
    write_yy_push_accepting_state(&mut file)?;
    write_yy_pop_accepting_state(&mut file)?;
    write_yy_add_buffer(&mut file)?;
    write_yy_if_no_match(&mut file)?;
    write_yy_set_yytext(&mut file)?;
    write_yy_next_char(&mut file)?;
    write_yymore(&mut file)?;


    yy_action(&file_parts, &mut file)?;
    write_yy_if_match(&mut file)?;
    write_yy_reject(&mut file)?;

    yy_final(&file_parts, &mut file)?;
    create_yy_search_final(file_parts.actions(), &mut file)?;

    write_yylex(file_parts.in_yylex(), &mut file)?;

    // Write user routine
    write_user_routine(file_parts.user_routine(), &mut file)?;

    Ok(())
}
