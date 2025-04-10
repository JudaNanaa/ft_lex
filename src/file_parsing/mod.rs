use std::{iter::Peekable, str::Chars};

use crate::regex::NFA;

pub mod parsing;
mod definitions;
mod rules;
mod user_routine;

#[derive(PartialEq)]
enum FileState {
	Definition,
	Rules,
	UserRoutine,
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