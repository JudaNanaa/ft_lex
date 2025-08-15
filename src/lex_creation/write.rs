use std::{
    fs::File,
    io::{Read, Write},
};

use crate::file_parsing::definitions::Definition;

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

pub fn write_variables(definitions: &[Definition], file: &mut File) -> std::io::Result<()> {
    let mut file_content = open_template_file(VARIABLES)?;

    let mut to_add = String::new();

    for elem in definitions {
        match elem {
            Definition::Bloc { content } | Definition::LineWithSpace { content } => {
                to_add.push_str(&content);
                to_add.push('\n');
            }
            _ => {}
        }
    }

    file_content = file_content.replace("change_me_in_variables!", &to_add);

    file.write_all(file_content.as_bytes())?;
    return Ok(());
}

pub fn write_user_routine(user_routine: &str, file: &mut File) -> std::io::Result<()> {
    file.write_all(user_routine.as_bytes())?;
    return Ok(());
}
