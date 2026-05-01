use crate::file_parsing::FilePart;

use super::combine::process_and_combine_rules;
use super::parsing::extract_yytext_mode;
use super::rules::rules::map_actions;

pub fn merge_file_parts(parts: Vec<FilePart>) -> Result<FilePart, String> {
    let mut combined_definitions = Vec::new();
    let mut combined_in_yylex = Vec::new();
    let mut combined_user_routine = String::new();
    let mut combined_rules = Vec::new();

    for part in parts {
        combined_definitions.extend(part.definitions);
        combined_in_yylex.extend(part.in_yylex);
        combined_user_routine.push_str(&part.user_routine);
        combined_rules.extend(part.rule_action);
    }

    let combined_map_actions = map_actions(&combined_rules);
    let yytext_mode = extract_yytext_mode(&combined_definitions);
    let (dfa, actions, rule_action) = process_and_combine_rules(combined_rules)?;

    Ok(FilePart {
        definitions: combined_definitions,
        in_yylex: combined_in_yylex,
        dfa,
        rule_action,
        actions,
        map_actions: combined_map_actions,
        user_routine: combined_user_routine,
        yytext_mode,
    })
}
