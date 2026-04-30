use std::fs::File;

use crate::file_parsing::FilePart;

use super::{
    yy_accept::yy_accept, yy_ec::create_yy_ec, yy_nxt::create_yy_nxt, yy_trailing::yy_trailing,
    yy_trailing_accept::yy_trailing_accept,
};

pub fn tables_creation(file_parts: &FilePart, file: &mut File) -> std::io::Result<()> {
    let eq_hash = create_yy_ec(file_parts.dfa().charset(), file)?;

    create_yy_nxt(file_parts.dfa(), &eq_hash, file)?;

    yy_accept(file_parts.dfa(), file)?;
    yy_trailing(file_parts.dfa(), file)?;
    yy_trailing_accept(file_parts.dfa(), file)?;

    Ok(())
}
