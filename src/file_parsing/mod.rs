use std::{collections::HashMap, iter::Peekable, str::Chars};

use crate::{
    file_parsing::{definitions::Definition, rules::RuleAction},
    regex::dfa::DFA,
};

mod combine;
pub mod definitions;
pub mod parsing;
mod rules;
mod user_routine;

pub struct FileInfo<'a> {
    it: Peekable<Chars<'a>>,
    line_nb: usize,
    name: &'a str,
}

#[derive(Debug)]
pub struct FilePart {
    definitions: Vec<Definition>,
    in_yylex: Vec<String>,
    dfa: DFA,
    rule_action: Vec<RuleAction>,
    actions: HashMap<usize, Vec<String>>,
    map_actions: HashMap<String, usize>,
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
    pub fn map_actions(&self) -> &HashMap<String, usize> {
        return &self.map_actions;
    }
    pub fn in_yylex(&self) -> &[String] {
        return &self.in_yylex;
    }
    pub fn definitions(&self) -> &[Definition] {
        return &self.definitions;
    }
    pub fn rule_action(&self) -> &[RuleAction] {
        return &self.rule_action;
    }
}
