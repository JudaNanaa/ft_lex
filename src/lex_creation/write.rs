use std::{
    fs::File,
    io::{Read, Write},
};

use crate::file_parsing::Definition;

use super::{
    DEFINES, INCLUDES, VARIABLES, YYLESS, YYLEX, YYMORE, YY_ADD_BUFFER, YY_FATAL_ERROR, YY_IF_MATCH, YY_IF_NO_MATCH, YY_INCREASE_ACCEPTING_STACK_LEN, YY_INIT_ACCEPTING_STACK, YY_INIT_BUFFER, YY_NEXT_CHAR, YY_POP_ACCEPTING_STATE, YY_PUSH_ACCEPTING_STATE, YY_REJECT, YY_SET_YYTEXT
};

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

pub fn write_yy_add_buffer(file: &mut File) -> std::io::Result<()> {
    let file_content = open_template_file(YY_ADD_BUFFER)?;
    file.write_all(file_content.as_bytes())?;
    return Ok(());
}

pub fn write_yy_fatal_error(file: &mut File) -> std::io::Result<()> {
    let file_content = open_template_file(YY_FATAL_ERROR)?;
    file.write_all(file_content.as_bytes())?;
    return Ok(());
}

pub fn write_yy_if_match(file: &mut File) -> std::io::Result<()> {
    let file_content = open_template_file(YY_IF_MATCH)?;
    file.write_all(file_content.as_bytes())?;
    return Ok(());
}

pub fn write_yy_if_no_match(file: &mut File) -> std::io::Result<()> {
    let file_content = open_template_file(YY_IF_NO_MATCH)?;
    file.write_all(file_content.as_bytes())?;
    return Ok(());
}

pub fn write_yy_increase_accepting_stack_len(file: &mut File) -> std::io::Result<()> {
    let file_content = open_template_file(YY_INCREASE_ACCEPTING_STACK_LEN)?;
    file.write_all(file_content.as_bytes())?;
    return Ok(());
}

pub fn write_yy_init_accepting_stack(file: &mut File) -> std::io::Result<()> {
    let file_content = open_template_file(YY_INIT_ACCEPTING_STACK)?;
    file.write_all(file_content.as_bytes())?;
    return Ok(());
}

pub fn write_yy_init_buffer(file: &mut File) -> std::io::Result<()> {
    let file_content = open_template_file(YY_INIT_BUFFER)?;
    file.write_all(file_content.as_bytes())?;
    return Ok(());
}

pub fn write_yylex(in_yylex: &[String], file: &mut File) -> std::io::Result<()> {
    let mut file_content = open_template_file(YYLEX)?;

    let mut to_add = String::new();

    for elem in in_yylex {
        to_add.push_str(&elem);
    }

    dbg!(&to_add);

    file_content = file_content.replace("change_me_in_yylex!", &to_add);

    file.write_all(file_content.as_bytes())?;
    return Ok(());
}

pub fn write_yy_next_char(file: &mut File) -> std::io::Result<()> {
    let file_content = open_template_file(YY_NEXT_CHAR)?;
    file.write_all(file_content.as_bytes())?;
    return Ok(());
}

pub fn write_yy_pop_accepting_state(file: &mut File) -> std::io::Result<()> {
    let file_content = open_template_file(YY_POP_ACCEPTING_STATE)?;
    file.write_all(file_content.as_bytes())?;
    return Ok(());
}

pub fn write_yy_push_accepting_state(file: &mut File) -> std::io::Result<()> {
    let file_content = open_template_file(YY_PUSH_ACCEPTING_STATE)?;
    file.write_all(file_content.as_bytes())?;
    return Ok(());
}

pub fn write_yy_reject(file: &mut File) -> std::io::Result<()> {
    let file_content = open_template_file(YY_REJECT)?;
    file.write_all(file_content.as_bytes())?;
    return Ok(());
}

pub fn write_yymore(file: &mut File) -> std::io::Result<()> {
    let file_content = open_template_file(YYMORE)?;
    file.write_all(file_content.as_bytes())?;
    return Ok(());
}

pub fn write_yyless(file: &mut File) -> std::io::Result<()> {
    let file_content = open_template_file(YYLESS)?;
    file.write_all(file_content.as_bytes())?;
    return Ok(());
}

pub fn write_yy_set_yytext(file: &mut File) -> std::io::Result<()> {
    let file_content = open_template_file(YY_SET_YYTEXT)?;
    file.write_all(file_content.as_bytes())?;
    return Ok(());
}

pub fn write_user_routine(user_routine: &str, file: &mut File) -> std::io::Result<()> {
    file.write_all(user_routine.as_bytes())?;
    return Ok(());
}
