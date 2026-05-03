pub mod backend;
pub mod c;
pub mod creation;
mod functions;
pub mod stats;
mod tables;
mod write;

const INCLUDES: &str = "src/lex_creation/templates/includes.c";
const DEFINES: &str = "src/lex_creation/templates/defines.c";
const VARIABLES: &str = "src/lex_creation/templates/variables.c";
const YYLEX: &str = "src/lex_creation/templates/yylex.c";

pub(crate) const SPACE: &str = "    ";
