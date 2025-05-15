use std::{collections::HashMap, iter::Peekable, str::Chars};

use crate::regex::{dfa::DFA, NFA};

mod combine;
mod definitions;
pub mod parsing;
mod rules;
mod user_routine;

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
    actions: HashMap<usize, Vec<String>>,
    action_hash: HashMap<String, usize>,
    user_routine: String,
}

impl FilePart {
    pub fn user_routine(&self) -> &str {
        return &self.user_routine;
    }
    pub fn dfa(&self) -> &DFA {
        return &self.dfa;
    }
    pub fn actions(&self) -> &HashMap<usize, Vec<String>> {
        return &self.actions;
    }
    pub fn action_hash(&self) -> &HashMap<String, usize> {
        return &self.action_hash;
    }
    pub fn in_yylex(&self) -> &[String] {
        return &self.in_yylex;
    }
    pub fn definitions(&self) -> &[Definition] {
        return &self.definitions;
    }
}
