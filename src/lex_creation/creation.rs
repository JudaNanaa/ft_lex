use std::fs::File;

use crate::{
    file_parsing::FilePart,
    lex_creation::{functions::yy_action::yy_action, tables::table::tables_creation, write::{write_defines, write_includes, write_user_routine, write_variables}, LEX_FILE
    },
};

pub fn lex_creation(file_parts: FilePart) -> std::io::Result<()> {
    let mut file = File::create(LEX_FILE)?;

    write_includes(&mut file)?;
    write_defines(&mut file)?;
    write_variables(&mut file)?;

	tables_creation(&file_parts, &mut file)?;
	yy_action(&file_parts, &mut file)?;

    // Write user routine
    write_user_routine(file_parts.user_routine(), &mut file)?;

    todo!();
}
