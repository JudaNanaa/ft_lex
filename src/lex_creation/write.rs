use std::{
    fs::File,
    io::{Read, Write},
};

use super::{DEFINES, INCLUDES, VARIABLES};

fn open_template_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;

    let mut file_content = String::new();

    file.read_to_string(&mut file_content)?;

    return Ok(file_content);
}

pub fn write_includes(file: &mut File) -> std::io::Result<()> {
    let file_content = open_template_file(INCLUDES)?;
    file.write_all(file_content.as_bytes())?;
    return Ok(());
}

pub fn write_defines(file: &mut File) -> std::io::Result<()> {
    let file_content = open_template_file(DEFINES)?;
    file.write_all(file_content.as_bytes())?;
    return Ok(());
}

pub fn write_variables(file: &mut File) -> std::io::Result<()> {
    let file_content = open_template_file(VARIABLES)?;
    file.write_all(file_content.as_bytes())?;
    return Ok(());
}

pub fn write_user_routine(user_routine: &str, file: &mut File) -> std::io::Result<()> {
    file.write_all(user_routine.as_bytes())?;
    return Ok(());
}
