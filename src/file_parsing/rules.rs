use std::{iter::Peekable, slice::Iter, str::Chars};

use crate::{
    file_parsing::rules,
    regex::{nfa::nfa::construct_nfa, regex_tokenizer},
};

use super::{DefinitionToken, FileInfo, RuleAction};

pub fn parse_rules_part(
    file: &mut FileInfo,
    definitions: Vec<DefinitionToken>,
) -> Result<Vec<RuleAction, String>> {
    let rules = Vec::new();

    while let Some(char) = file.it.next() {
        match char {
            '%' => {
                if let Some(c) = file.it.peek() {
                    if c == '%' {
                        file.it.next();
                        return Ok(rules);
                    }
                }
            }

            _ => todo!(),
        }
    }

    todo!();
}
