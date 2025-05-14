use std::fs::File;

use crate::file_parsing::FilePart;

use super::{add_action::add_action, yy_accept::yy_accept, yy_ec::create_yy_ec, yy_nxt::create_yy_nxt};

fn create_yylex(transition_table: &Vec<Vec<usize>>, file: &mut File) -> std::io::Result<()> {


	
	todo!()
}

pub fn tables_creation(file_parts: &FilePart, file: &mut File) -> std::io::Result<()> {

	let eq_hash = create_yy_ec(file_parts.dfa().charset(), file)?;

	let transition_table = create_yy_nxt(file_parts.dfa(), &eq_hash, file)?;


	let accept_table = yy_accept(file_parts.dfa(), file)?;

	add_action(file_parts.actions(), file)?;
	// create_yylex(&transition_table, file)?;
    return Ok(());
}


