mod tables;
use tables::{write_accept_actions_rust, write_tables_rust};

use crate::{
    file_parsing::{definitions::Definition, FilePart, YytextMode},
    lex_creation::backend::CodegenBackend,
};

pub struct RustBackend;

impl RustBackend {
    pub fn new() -> Self {
        RustBackend
    }
}

impl CodegenBackend for RustBackend {
    fn write_header(&self, _: &[Definition], _: YytextMode, _: &mut dyn std::io::Write) -> std::io::Result<()> {
        todo!()
    }

    fn write_yytext_section(&self, _: YytextMode, _: &mut dyn std::io::Write) -> std::io::Result<()> {
        todo!()
    }

    fn write_tables(&self, file_parts: &FilePart, out: &mut dyn std::io::Write) -> std::io::Result<()> {
        write_tables_rust(file_parts, out)
    }

    fn write_is_exclusive_state(&self, _: &FilePart, _: &mut dyn std::io::Write) -> std::io::Result<()> {
        todo!()
    }

    fn write_action(&self, _: &FilePart, _: &mut dyn std::io::Write) -> std::io::Result<()> {
        todo!()
    }

    fn write_accept_actions(&self, file_parts: &FilePart, out: &mut dyn std::io::Write) -> std::io::Result<()> {
        write_accept_actions_rust(file_parts, out)
    }

    fn write_yylex(&self, _: &[String], _: YytextMode, _: &mut dyn std::io::Write) -> std::io::Result<()> {
        todo!()
    }

    fn write_user_routine(&self, _: &str, _: &mut dyn std::io::Write) -> std::io::Result<()> {
        todo!()
    }

    fn output_filename(&self) -> &str {
        "ft_lex.yy.rs"
    }
}
