use std::collections::LinkedList;

#[derive(Debug)]
pub struct Regex {
	value: char,
}

pub fn create_regex_list(regex: String) -> LinkedList<Regex>
{
	let mut regex_list: LinkedList<Regex> = LinkedList::new();
	for char in regex.chars() {
		regex_list.push_back(Regex {value: char});
	}
	return regex_list;
}