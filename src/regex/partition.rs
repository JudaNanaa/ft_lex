use crate::regex::tokenizer::extract_charset_hashset;
use crate::regex::utils::expand_escape;
use std::collections::{HashMap, HashSet};

pub fn collect_charsets_from_regex(regex: &str) -> Vec<HashSet<char>> {
    let mut charsets: Vec<HashSet<char>> = Vec::new();
    let mut chars = regex.chars();

    while let Some(c) = chars.next() {
        match c {
            '[' => {
                let set = extract_charset_hashset(&mut chars);
                charsets.push(set);
            }
            '.' => {
                let set: HashSet<char> = (0..=255u8)
                    .filter_map(|c| char::from_u32(u32::from(c)))
                    .filter(|&c| c != '\n')
                    .collect();
                charsets.push(set);
            }
            '\\' => {
                let set = if let Some(escaped) = expand_escape(&mut chars) {
                    let mut s = HashSet::new();
                    s.insert(escaped);
                    s
                } else {
                    let mut s = HashSet::new();
                    s.insert('\\');
                    s
                };
                charsets.push(set);
            }
            // NOTE: collect_charsets_from_regex receives raw regex strings
            // (pre-NFA, post-macro-expansion). The tokenizer's get_string_under_quotes
            // (src/regex/tokenizer/quotes.rs) converts escape sequences like \n to their
            // actual characters before building the NFA, but that path produces Tokens, not
            // regex strings. Here we receive the raw source string, so "..." content is
            // iterated character-by-character as written. For partition refinement purposes
            // this is correct: each raw character (e.g. '\' and 'n' in a literal "\n")
            // forms its own singleton charset, which still participates correctly in
            // equivalence-class splitting.
            '"' => {
                for inner in chars.by_ref() {
                    if inner == '"' {
                        break;
                    }
                    let mut s = HashSet::new();
                    s.insert(inner);
                    charsets.push(s);
                }
            }
            '{' => {
                // Skip repetition counts like {3,5}
                for ch in chars.by_ref() {
                    if ch == '}' {
                        break;
                    }
                }
            }
            '*' | '+' | '?' | '|' | '(' | ')' | '/' => {
                // Operators — skip ('/' is the trailing-context operator in lex)
            }
            other => {
                let mut s = HashSet::new();
                s.insert(other);
                charsets.push(s);
            }
        }
    }

    charsets
}

pub fn partition_refinement(charsets: &[HashSet<char>]) -> (HashMap<char, usize>, usize) {
    if charsets.is_empty() {
        return (HashMap::new(), 0);
    }

    // Start with one partition containing all 256 chars
    let mut partition: Vec<HashSet<char>> = vec![(0..=255u8)
        .filter_map(|c| char::from_u32(u32::from(c)))
        .collect::<HashSet<char>>()];

    // Refine partition against each charset
    for s in charsets {
        // Only iterate over classes that existed at the start of this round
        let current_len = partition.len();
        for i in 0..current_len {
            let intersection: HashSet<char> = partition[i].intersection(s).copied().collect();
            let difference: HashSet<char> = partition[i].difference(s).copied().collect();
            if !intersection.is_empty() && !difference.is_empty() {
                partition[i] = intersection;
                partition.push(difference);
            }
        }
    }

    // Collect all "used chars": union of all chars in charsets
    let used_chars: HashSet<char> = charsets.iter().flat_map(|s| s.iter().copied()).collect();

    // Assign class indices: classes with at least one used char get 1, 2, 3, ...
    // Classes with only unused chars get index 0 (trap class)
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut next_index = 1usize;

    for class in &partition {
        let has_used = class.iter().any(|c| used_chars.contains(c));
        if has_used {
            let idx = next_index;
            next_index += 1;
            for &ch in class {
                map.insert(ch, idx);
            }
        }
        // chars in classes with no used chars get index 0 — not added to map
    }

    let num_classes = next_index - 1;
    (map, num_classes)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── collect_charsets_from_regex tests ─────────────────────────────────────

    /// Test 1: plain charset "[a-z]" → exactly 1 HashSet with all 26 lowercase letters.
    #[test]
    fn test_collect_plain_charset() {
        let charsets = collect_charsets_from_regex("[a-z]");
        assert_eq!(charsets.len(), 1, "expected exactly 1 charset for [a-z]");
        let set = &charsets[0];
        assert_eq!(set.len(), 26, "expected 26 chars in [a-z]");
        assert!(set.contains(&'a'), "'a' should be in [a-z]");
        assert!(set.contains(&'z'), "'z' should be in [a-z]");
    }

    /// Test 2: dot "." → 1 HashSet with 255 chars (all byte values except '\n').
    #[test]
    fn test_collect_dot() {
        let charsets = collect_charsets_from_regex(".");
        assert_eq!(charsets.len(), 1, "expected exactly 1 charset for '.'");
        let set = &charsets[0];
        assert_eq!(
            set.len(),
            255,
            "dot should match 255 chars (all except '\\n')"
        );
        assert!(!set.contains(&'\n'), "'\\n' must not be in dot charset");
        assert!(set.contains(&'a'), "'a' should be in dot charset");
    }

    /// Test 3: literal "abc" → 3 HashSets: {a}, {b}, {c}.
    #[test]
    fn test_collect_literal_chars() {
        let charsets = collect_charsets_from_regex("abc");
        assert_eq!(charsets.len(), 3, "expected 3 charsets for 'abc'");
        assert!(charsets[0].contains(&'a') && charsets[0].len() == 1);
        assert!(charsets[1].contains(&'b') && charsets[1].len() == 1);
        assert!(charsets[2].contains(&'c') && charsets[2].len() == 1);
    }

    // ── partition_refinement tests ────────────────────────────────────────────

    /// Test 4: empty input → (HashMap::new(), 0).
    #[test]
    fn test_partition_refinement_empty() {
        let (map, num_classes) = partition_refinement(&[]);
        assert!(map.is_empty(), "map should be empty for empty input");
        assert_eq!(num_classes, 0, "num_classes should be 0 for empty input");
    }

    /// Test 5: single charset {a..z} → all a..z map to the same class index; no other
    /// chars appear in the map (they fall into the trap/unused class 0).
    #[test]
    fn test_partition_refinement_single_charset() {
        let az: HashSet<char> = ('a'..='z').collect();
        let charsets = vec![az];
        let (map, num_classes) = partition_refinement(&charsets);

        // There should be exactly 1 non-trap class
        assert_eq!(num_classes, 1, "expected 1 equivalence class for {{a..z}}");

        // All a..z must be present in the map and share the same index
        let idx = *map.get(&'a').expect("'a' must be in map");
        assert!(idx >= 1, "class index must be >= 1");
        for ch in 'a'..='z' {
            assert_eq!(
                map.get(&ch).copied(),
                Some(idx),
                "all a..z must map to the same class index"
            );
        }

        // Characters outside a..z must not appear in the map
        assert!(!map.contains_key(&'A'), "'A' should not be in map");
        assert!(!map.contains_key(&'0'), "'0' should not be in map");
    }

    /// Test 6: two charsets {a..z} and {a} → 'a' must have a different class index
    /// from 'b'..'z', and both classes appear in the map.
    #[test]
    fn test_partition_refinement_two_charsets() {
        let az: HashSet<char> = ('a'..='z').collect();
        let just_a: HashSet<char> = std::iter::once('a').collect();
        let charsets = vec![az, just_a];
        let (map, num_classes) = partition_refinement(&charsets);

        // Expect 2 non-trap classes: {a} and {b..z}
        assert_eq!(num_classes, 2, "expected 2 equivalence classes");

        let idx_a = *map.get(&'a').expect("'a' must be in map");
        let idx_b = *map.get(&'b').expect("'b' must be in map");
        assert_ne!(
            idx_a, idx_b,
            "'a' and 'b' must be in different equivalence classes"
        );

        // All of b..z must share the same class as 'b'
        for ch in 'b'..='z' {
            assert_eq!(
                map.get(&ch).copied(),
                Some(idx_b),
                "all b..z must be in the same class as 'b'"
            );
        }

        // Characters outside a..z should not be in the map
        assert!(!map.contains_key(&'A'), "'A' should not be in map");
    }

    /// Test 7: trailing-context '/' in "abc/def" must be treated as an operator
    /// (skipped), so collect_charsets_from_regex returns 6 singletons for a,b,c,d,e,f
    /// with no '/' charset.
    #[test]
    fn test_collect_trailing_context_slash() {
        let charsets = collect_charsets_from_regex("abc/def");
        assert_eq!(
            charsets.len(),
            6,
            "expected 6 charsets for 'abc/def' (slash is an operator, not a literal)"
        );

        // Verify the expected characters appear as singletons
        let expected_chars = ['a', 'b', 'c', 'd', 'e', 'f'];
        for (i, &expected) in expected_chars.iter().enumerate() {
            assert!(
                charsets[i].contains(&expected) && charsets[i].len() == 1,
                "charset[{}] should be {{'{}'}}",
                i,
                expected
            );
        }

        // Verify '/' never appears in any charset
        for set in &charsets {
            assert!(
                !set.contains(&'/'),
                "'/' must not appear in any charset (it is an operator)"
            );
        }
    }
}
