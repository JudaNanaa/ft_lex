pub mod creation;
mod functions;
mod tables;
mod write;

const LEX_FILE: &str = "ft_lex.yy.c";
const INCLUDES: &str = "src/lex_creation/templates/includes.c";
const YYMORE: &str = "src/lex_creation/templates/yymore.c";
const DEFINES: &str = "src/lex_creation/templates/defines.c";
const VARIABLES: &str = "src/lex_creation/templates/variables.c";
const YY_ADD_BUFFER: &str = "src/lex_creation/templates/yy_add_buffer.c";
const YY_FATAL_ERROR: &str = "src/lex_creation/templates/yy_fatal_error.c";
const YY_IF_MATCH: &str = "src/lex_creation/templates/yy_if_match.c";
const YY_IF_NO_MATCH: &str = "src/lex_creation/templates/yy_if_no_match.c";
const YY_INCREASE_ACCEPTING_STACK_LEN: &str =
    "src/lex_creation/templates/yy_increase_accepting_stack_len.c";
const YY_INIT_ACCEPTING_STACK: &str = "src/lex_creation/templates/yy_init_accepting_stack.c";
const YY_INIT_BUFFER: &str = "src/lex_creation/templates/yy_init_buffer.c";
const YYLEX: &str = "src/lex_creation/templates/yylex.c";
const YY_NEXT_CHAR: &str = "src/lex_creation/templates/yy_next_char.c";
const YY_POP_ACCEPTING_STATE: &str = "src/lex_creation/templates/yy_pop_accepting_state.c";
const YY_PUSH_ACCEPTING_STATE: &str = "src/lex_creation/templates/yy_push_accepting_state.c";
const YY_REJECT: &str = "src/lex_creation/templates/yy_reject.c";
const YY_SET_YYTEXT: &str = "src/lex_creation/templates/yy_set_yytext.c";
const YYLESS: &str = "src/lex_creation/templates/yyless.c";
const INPUT: &str = "src/lex_creation/templates/input.c";
const UNPUT: &str = "src/lex_creation/templates/unput.c";
const YYWRAP: &str = "src/lex_creation/templates/yywrap.c";

const SPACE: &str = "    ";
