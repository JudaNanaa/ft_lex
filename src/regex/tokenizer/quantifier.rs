use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Quantifier {
    Equal(usize),
    AtLeast(usize),
    Range(usize, usize),
}

fn is_numeric_string(input: &String) -> bool {
    input.chars().all(|c| c.is_numeric())
}

fn parse_range(range_str: &str) -> Quantifier {
    let parts: Vec<&str> = range_str.split(',').collect();

    if parts.len() > 2 {
        panic!("Unrecognized rule");
    }

    let min_repeats = parts[0];
    if min_repeats.is_empty() || !is_numeric_string(&min_repeats.to_string()) {
        panic!("Invalid numeric value inside {}", "{}")
    }

    let min_value: usize = min_repeats.parse().unwrap();

    if parts.len() == 1 {
        return Quantifier::Equal(min_value);
    }

    let max_repeats = parts[1];
    if max_repeats.is_empty() {
        return Quantifier::AtLeast(min_value);
    }
    if !is_numeric_string(&max_repeats.to_string()) {
        panic!("Invalid numeric value inside {}", "{}")
    }

    let max_value: usize = max_repeats.parse().unwrap();
    if min_value > max_value as usize {
        panic!("Invalid range: min cannot be greater than max")
    }
    return Quantifier::Range(min_value, max_value);
}

pub fn extract_repetition_range(chars: &mut Chars<'_>) -> Quantifier {
    let mut range_str = String::new();

    while let Some(c) = chars.next() {
        if c == '}' {
            break;
        }
        range_str.push(c);
    }

    if range_str.is_empty() {
        panic!("Unrecognized rule")
    }

    return parse_range(&range_str);
}
