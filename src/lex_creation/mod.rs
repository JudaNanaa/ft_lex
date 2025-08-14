pub mod creation;
mod functions;
mod tables;
mod write;

const LEX_FILE: &str = "ft_lex.yy.c";
const INCLUDES: &str = "src/lex_creation/templates/includes.c";
const DEFINES: &str = "src/lex_creation/templates/defines.c";
const VARIABLES: &str = "src/lex_creation/templates/variables.c";

const SPACE: &str = "    ";
