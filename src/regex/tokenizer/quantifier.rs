use std::str::Chars;

use super::Quantifier;

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

// -------------------- Tests

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::Chars;

    fn get_chars(s: &str) -> Chars<'_> {
        s.chars()
    }

    #[test]
    fn test_equal_quantifier() {
        let mut chars = get_chars("3}");
        let quant = extract_repetition_range(&mut chars);
        assert_eq!(quant, Quantifier::Equal(3));
    }

    #[test]
    fn test_at_least_quantifier() {
        let mut chars = get_chars("2,}");
        let quant = extract_repetition_range(&mut chars);
        assert_eq!(quant, Quantifier::AtLeast(2));
    }

    #[test]
    fn test_range_quantifier() {
        let mut chars = get_chars("1,4}");
        let quant = extract_repetition_range(&mut chars);
        assert_eq!(quant, Quantifier::Range(1, 4));
    }

    #[test]
    #[should_panic(expected = "Unrecognized rule")]
    fn test_empty_range_should_panic() {
        let mut chars = get_chars("}");
        let _ = extract_repetition_range(&mut chars);
    }

    #[test]
    #[should_panic(expected = "Invalid numeric value")]
    fn test_invalid_min_should_panic() {
        let mut chars = get_chars("a,3}");
        let _ = extract_repetition_range(&mut chars);
    }

    #[test]
    #[should_panic(expected = "Invalid numeric value")]
    fn test_invalid_max_should_panic() {
        let mut chars = get_chars("2,b}");
        let _ = extract_repetition_range(&mut chars);
    }

    #[test]
    #[should_panic(expected = "Invalid range")]
    fn test_min_greater_than_max_should_panic() {
        let mut chars = get_chars("5,2}");
        let _ = extract_repetition_range(&mut chars);
    }

    #[test]
    #[should_panic(expected = "Unrecognized rule")]
    fn test_too_many_commas_should_panic() {
        let mut chars = get_chars("1,2,3}");
        let _ = extract_repetition_range(&mut chars);
    }
}
