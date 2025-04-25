use std::{
    fs::File,
    io::{Read, Write},
};

use crate::{
    file_parsing::FilePart,
    lex_creation::{INCLUDES, LEX_FILE},
};

fn open_template_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;

    let mut file_content = String::new();

    file.read_to_string(&mut file_content)?;

    return Ok(file_content);
}

pub fn lex_creation(file_parts: FilePart) -> std::io::Result<()> {
    let mut file = File::create(LEX_FILE)?;

    let file_content = open_template_file(INCLUDES)?;

    file.write_all(file_content.as_bytes())?;
    // writeln!(&mut file, )?;
    todo!();
}
