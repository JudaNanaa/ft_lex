

pub fn expand_escape(c: char) -> char {
	match c {
		'n' => return '\n',
		't' => return '\t',
		'r' => return '\r',
		_ => return c,
	}
}