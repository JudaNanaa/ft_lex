use std::{iter::Peekable, str::Chars};

use crate::regex::NFA;

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
pub enum DefinitionToken {
    Bloc { content: String },
    LineWithSpace { content: String },
    Definition { name: String, value: String },
    InclusiveState { names: Vec<String> },
    ExclusiveState { names: Vec<String> },
}

struct RuleAction {
    nfa: NFA,
    action: String,
}

pub struct FileInfo<'a> {
    it: Peekable<Chars<'a>>,
    line_nb: usize,
    name: &'a str,
}
