use std::io;

use crate::file_parsing::{definitions::Definition, FilePart, YytextMode};

pub trait CodegenBackend {
    fn write_header(
        &self,
        definitions: &[Definition],
        mode: YytextMode,
        out: &mut dyn io::Write,
    ) -> io::Result<()>;

    fn write_yytext_section(
        &self,
        mode: YytextMode,
        out: &mut dyn io::Write,
    ) -> io::Result<()>;

    fn write_tables(
        &self,
        file_parts: &FilePart,
        out: &mut dyn io::Write,
    ) -> io::Result<()>;

    fn write_is_exclusive_state(
        &self,
        file_parts: &FilePart,
        out: &mut dyn io::Write,
    ) -> io::Result<()>;

    fn write_action(
        &self,
        file_parts: &FilePart,
        out: &mut dyn io::Write,
    ) -> io::Result<()>;

    fn write_accept_actions(
        &self,
        file_parts: &FilePart,
        out: &mut dyn io::Write,
    ) -> io::Result<()>;

    fn write_yylex(
        &self,
        in_yylex: &[String],
        mode: YytextMode,
        out: &mut dyn io::Write,
    ) -> io::Result<()>;

    fn write_user_routine(
        &self,
        user_routine: &str,
        out: &mut dyn io::Write,
    ) -> io::Result<()>;

    fn output_filename(&self) -> &str;
}
