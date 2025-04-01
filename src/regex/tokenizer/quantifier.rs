use std::str::Chars;

fn is_numeric_string(input: &String) -> bool {
    input.chars().all(|c| c.is_numeric())
}

fn parse_range(range_str: &str) -> (u32, Option<i32>) {
    let parts: Vec<&str> = range_str.split(',').collect();
    
	if parts.len() > 2 {
		panic!("Unrecognized rule");
	}

    let min_repeats = parts[0];
    if min_repeats.is_empty() || !is_numeric_string(&min_repeats.to_string()) {
        panic!("Invalid numeric value inside {}", "{}")
    }

    let min_value: u32 = min_repeats.parse().unwrap();
    
    if parts.len() == 1 {
        return (min_value, None);
    }
    
    let max_repeats = parts[1];
    if max_repeats.is_empty() {
        return (min_value, Some(-1));
    }
    if !is_numeric_string(&max_repeats.to_string()) {
        panic!("Invalid numeric value inside {}", "{}")
    }
    
    let max_value: i32 = max_repeats.parse().unwrap();
    if min_value > max_value as u32 {
        panic!("Invalid range: min cannot be greater than max")
    }
    
    return (min_value, Some(max_value));
}

pub fn extract_repetition_range(chars: &mut Chars<'_>) -> (u32, Option<i32>) {
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
