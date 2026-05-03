mod functions;
mod tables;
pub mod write;

use crate::{
    file_parsing::{definitions::Definition, FilePart, YytextMode},
    lex_creation::backend::CodegenBackend,
};
use functions::{yy_action::yy_action, yy_is_exclusive_state::write_yy_is_exclusive_state};
use std::io;
use tables::{write_accept_actions_c, write_tables_c};

pub struct CBackend;

impl CBackend {
    pub fn new() -> Self {
        CBackend
    }
}

impl CodegenBackend for CBackend {
    fn write_header(
        &self,
        definitions: &[Definition],
        mode: YytextMode,
        out: &mut dyn io::Write,
    ) -> io::Result<()> {
        write::write_includes(out)?;
        write::write_defines(out, definitions, mode)?;
        write::write_variables(definitions, out)
    }

    fn write_yytext_section(&self, mode: YytextMode, out: &mut dyn io::Write) -> io::Result<()> {
        write::write_yytext_section(mode, out)
    }

    fn write_tables(&self, file_parts: &FilePart, out: &mut dyn io::Write) -> io::Result<()> {
        write_tables_c(file_parts, out)
    }

    fn write_is_exclusive_state(
        &self,
        file_parts: &FilePart,
        out: &mut dyn io::Write,
    ) -> io::Result<()> {
        write_yy_is_exclusive_state(file_parts, out)
    }

    fn write_action(&self, file_parts: &FilePart, out: &mut dyn io::Write) -> io::Result<()> {
        yy_action(file_parts, out)
    }

    fn write_accept_actions(
        &self,
        file_parts: &FilePart,
        out: &mut dyn io::Write,
    ) -> io::Result<()> {
        write_accept_actions_c(file_parts, out)
    }

    fn write_yylex(
        &self,
        in_yylex: &[String],
        mode: YytextMode,
        out: &mut dyn io::Write,
    ) -> io::Result<()> {
        write::write_yylex(out, in_yylex, mode)
    }

    fn write_user_routine(&self, user_routine: &str, out: &mut dyn io::Write) -> io::Result<()> {
        write::write_user_routine(user_routine, out)
    }

    fn output_filename(&self) -> &'static str {
        "ft_lex.yy.c"
    }
}
