use std::{collections::HashMap, iter::Peekable, str::Chars};

use crate::regex::{dfa::{State, DFA}, NFA};

mod combine;
mod definitions;
pub mod parsing;
mod rules;
mod user_routine;

#[derive(PartialEq)]
enum FileState {
    Definition,
    Rules,
    UserRoutine,
}

#[derive(Debug)]
pub enum Definition {
    Bloc { content: String },
    LineWithSpace { content: String },
    Definition { name: String, value: String },
    InclusiveState { names: Vec<String> },
    ExclusiveState { names: Vec<String> },
}

#[derive(Debug, Clone)]
struct RuleAction {
    nfa: NFA,
    action: String,
}

struct FileInfo<'a> {
    it: Peekable<Chars<'a>>,
    line_nb: usize,
    name: &'a str,
}

#[derive(Debug)]
pub struct FilePart {
    definitions: Vec<Definition>,
    in_yylex: Vec<String>,
    dfa: DFA,
    actions: HashMap<State, Vec<String>>,
    user_routine: String,
}
