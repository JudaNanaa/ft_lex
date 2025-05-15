use std::{char, collections::HashMap};

use crate::regex::{nfa::nfa::construct_nfa, regex_tokenizer};

use super::{Definition, FileInfo, RuleAction};

pub fn action_hash(rules: &Vec<RuleAction>) -> HashMap<String, usize> {

	let mut hash = HashMap::new();
	let mut index = 1;

	for rule in rules.iter() {
		if !hash.contains_key(&rule.action) && rule.action != "|" {
			hash.insert(rule.action.clone(), index);

			index += 1;
		}
	}

	return hash;
}

/// Ajoute tout le contenu entre guillemets dans `rule`.
fn append_quoted_string(rule: &mut String, file: &mut FileInfo) -> Result<(), &'static str> {
    rule.push('"');
    while let Some(ch) = file.it.next() {
        match ch {
            '\n' => {
                file.line_nb += 1;
                return Err("missing quote");
            }
            '\\' => {
                if let Some(escaped) = file.it.next() {
                    if escaped == '\n' {
                        file.line_nb += 1;
                        return Err("missing quote");
                    }
                    rule.push('\\');
                    rule.push(escaped);
                } else {
                    return Err("missing quote");
                }
            }
            '"' => {
                rule.push('"');
                break;
            }
            _ => rule.push(ch),
        }
    }
    return Ok(());
}

/// Remplace une référence à une définition par sa valeur.
fn resolve_definition(name: &str, definitions: &[Definition]) -> Result<String, &'static str> {
    if name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        return Ok(format!("{{{}}}", name));
    }

    for def in definitions {
        if let Definition::Definition {
            name: def_name,
            value,
        } = def
        {
            if *def_name == name {
                return Ok(format!("({})", value));
            }
        }
    }
    return Err("Definition not found");
}

/// Extrait le contenu entre accolades dans une règle.
fn extract_braced_definition(
    rule: &mut String,
    file: &mut FileInfo,
    defs: &[Definition],
) -> Result<(), &'static str> {
    let mut def_name = String::new();

    for ch in file.it.by_ref() {
        match ch {
            '\n' => {
                file.line_nb += 1;
                return Err("missing }");
            }
            '}' => {
                let replacement = resolve_definition(&def_name, defs)?;
                rule.push_str(&replacement);
                return Ok(());
            }
            _ => def_name.push(ch),
        }
    }
    return Err("unterminated brace block");
}

/// Extrait un ensemble de caractères entre crochets.
fn extract_character_class(rule: &mut String, file: &mut FileInfo) -> Result<(), &'static str> {
    rule.push('[');
    for ch in file.it.by_ref() {
        match ch {
            '\n' => {
                file.line_nb += 1;
                return Err("missing ]");
            }
            ']' => {
                rule.push(']');
                return Ok(());
            }
            _ => rule.push(ch),
        }
    }
    return Err("unterminated character class");
}

/// Extrait une action entre guillemets, en tenant compte des échappements.
fn read_quoted_action(file: &mut FileInfo, quote: char) -> Result<String, &'static str> {
    let mut result = String::new();
    result.push(quote);

    while let Some(ch) = file.it.next() {
        match ch {
            '\\' => {
                result.push('\\');
                if let Some(escaped) = file.it.next() {
                    result.push(escaped);
                }
            }
            '\n' => return Err("missing quote"),
            c if c == quote => {
                result.push(quote);
                return Ok(result);
            }
            _ => result.push(ch),
        }
    }

    return Err("unterminated quoted action");
}

/// Extrait une action encadrée par des `{}` (avec support de guillemets imbriqués).
fn read_braced_action(file: &mut FileInfo) -> Result<String, &'static str> {
    let mut result = String::new();
    result.push('{');

    while let Some(ch) = file.it.next() {
        match ch {
            '{' => result.push_str(&read_braced_action(file)?),
            '}' => {
                result.push('}');
                return Ok(result);
            }
            '\'' | '"' => result.push_str(&read_quoted_action(file, ch)?),
            _ => result.push(ch),
        }
    }

    return Err("unclosed brace in action");
}

/// Extrait le contenu complet d'une action après une règle.
fn parse_action(file: &mut FileInfo) -> Result<String, &'static str> {
    let mut action = String::new();

    while let Some(ch) = file.it.next() {
        match ch {
            '\n' => return Ok(action),
            '{' => action.push_str(&read_braced_action(file)?),
            '}' => todo!("error: unbalanced closing brace"),
            '\'' | '"' => action.push_str(&read_quoted_action(file, ch)?),
            _ => action.push(ch),
        }
    }

    return Err("unexpected EOF while reading action");
}

/// Coupe une ligne contenant une règle en deux : expression et action.
fn parse_rule_and_action(
    file: &mut FileInfo,
    first_char: char,
    defs: &[Definition],
) -> Result<(String, String), &'static str> {
    let mut rule = String::new();

    match first_char {
        '"' => append_quoted_string(&mut rule, file)?,
        '{' => extract_braced_definition(&mut rule, file, defs)?,
        '[' => extract_character_class(&mut rule, file)?,
        '}' | ']' => return Err("unexpected closing character"),
        _ => rule.push(first_char),
    }

    while let Some(ch) = file.it.peek().cloned() {
        match ch {
            '"' => {
                file.it.next();
                append_quoted_string(&mut rule, file)?;
            }
            ' ' | '\t' => {
                file.it.next();
                break;
            }
            '{' => {
                file.it.next();
                extract_braced_definition(&mut rule, file, defs)?;
            }
            '[' => {
                file.it.next();
                extract_character_class(&mut rule, file)?;
            }
            '}' | ']' => return Err("unexpected closing character"),
            _ => {
                file.it.next();
                rule.push(ch);
            }
        }
    }

    let action = parse_action(file)?.trim().to_string();
    return Ok((rule, action));
}

/// Parse la section des règles d'un fichier.
pub fn parse_rules_section(
    file: &mut FileInfo,
    definitions: &[Definition],
) -> Result<(Vec<RuleAction>, Vec<String>), String> {
    let mut texts = Vec::new();
    let mut rules = Vec::new();
    let mut next_state_id = 1;

    while let Some(ch) = file.it.next() {
        match ch {
            '\n' => {
                file.line_nb += 1;
                continue;
            }
            '%' => {
                if let Some('%') = file.it.peek().cloned() {
                    file.it.next();
                    return Ok((rules, texts));
                }
            }
            ' ' | '\t' => {
                let mut text = String::new();
                for ch in file.it.by_ref() {
                    if ch == '\n' {
                        file.line_nb += 1;
                        break;
                    }
                    text.push(ch);
                }
                texts.push(text);
            }
            _ => {
                let (rule_expr, action) = parse_rule_and_action(file, ch, definitions)?;
                let tokens = regex_tokenizer(&rule_expr);
                let (nfa, new_state_id) = construct_nfa(&tokens, next_state_id);
                next_state_id = new_state_id;
                rules.push(RuleAction { nfa, action });
            }
        }
    }

    return Ok((rules, texts));
}
