use std::{collections::HashMap, iter::Peekable, str::Chars};

use crate::{
    file_parsing::{definitions::Definition, rules::RuleAction},
    regex::dfa::DFA,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum YytextMode {
    Pointer,
    Array(usize),
}

mod combine;
pub mod definitions;
pub mod merge;
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
    pub(super) definitions: Vec<Definition>,
    pub(super) in_yylex: Vec<String>,
    pub(super) dfa: DFA,
    pub(super) rule_action: Vec<RuleAction>,
    pub(super) actions: HashMap<usize, Vec<String>>,
    pub(super) map_actions: HashMap<String, usize>,
    pub(super) user_routine: String,
    pub yytext_mode: YytextMode,
}

impl FilePart {
    pub fn user_routine(&self) -> &str {
        &self.user_routine
    }
    pub fn dfa(&self) -> &DFA {
        &self.dfa
    }
    pub fn actions(&self) -> &HashMap<usize, Vec<String>> {
        &self.actions
    }
    pub fn map_actions(&self) -> &HashMap<String, usize> {
        &self.map_actions
    }
    pub fn in_yylex(&self) -> &[String] {
        &self.in_yylex
    }
    pub fn definitions(&self) -> &[Definition] {
        &self.definitions
    }
    pub fn rule_action(&self) -> &[RuleAction] {
        &self.rule_action
    }
}
