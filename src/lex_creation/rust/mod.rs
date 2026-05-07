mod tables;
mod write;
use tables::write_tables_rust;
use write::{
    write_accept_actions_rust, write_action_rust, write_header_rust, write_is_exclusive_state_rust,
    write_user_routine_rust, write_yylex_rust, write_yytext_section_rust,
};

use crate::{
    file_parsing::{definitions::Definition, FilePart, YytextMode},
    lex_creation::backend::CodegenBackend,
};

pub struct RustBackend {
    compressed: bool,
}

impl RustBackend {
    pub fn new(compressed: bool) -> Self {
        RustBackend { compressed }
    }
}

impl CodegenBackend for RustBackend {
    fn write_header(
        &self,
        definitions: &[Definition],
        mode: YytextMode,
        out: &mut dyn std::io::Write,
    ) -> std::io::Result<()> {
        write_header_rust(definitions, mode, out)
    }
    fn write_yytext_section(
        &self,
        mode: YytextMode,
        out: &mut dyn std::io::Write,
    ) -> std::io::Result<()> {
        write_yytext_section_rust(mode, out)
    }
    fn write_tables(
        &self,
        file_parts: &FilePart,
        out: &mut dyn std::io::Write,
    ) -> std::io::Result<()> {
        write_tables_rust(file_parts, self.compressed, out)
    }
    fn write_is_exclusive_state(
        &self,
        file_parts: &FilePart,
        out: &mut dyn std::io::Write,
    ) -> std::io::Result<()> {
        write_is_exclusive_state_rust(file_parts, out)
    }
    fn write_action(
        &self,
        file_parts: &FilePart,
        out: &mut dyn std::io::Write,
    ) -> std::io::Result<()> {
        write_action_rust(file_parts, out)
    }
    fn write_accept_actions(
        &self,
        file_parts: &FilePart,
        out: &mut dyn std::io::Write,
    ) -> std::io::Result<()> {
        write_accept_actions_rust(file_parts, out)
    }
    fn write_yylex(
        &self,
        in_yylex: &[String],
        mode: YytextMode,
        out: &mut dyn std::io::Write,
    ) -> std::io::Result<()> {
        write_yylex_rust(in_yylex, mode, out)
    }
    fn write_user_routine(
        &self,
        user_routine: &str,
        out: &mut dyn std::io::Write,
    ) -> std::io::Result<()> {
        write_user_routine_rust(user_routine, out)
    }
    fn output_filename(&self) -> &'static str {
        "ft_lex_yy.rs"
    }
}
