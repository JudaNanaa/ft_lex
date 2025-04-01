// for strings
pub trait StringUtils {
    fn substr(&self, start: usize, len: usize) -> Self;
}

impl StringUtils for String {
    fn substr(&self, start: usize, len: usize) -> Self {
        return self.chars().skip(start).take(len).collect();
    }
}

pub fn expand_escape(c: char) -> char {
	match c {
		'n' => return '\n',
		't' => return '\t',
		'r' => return '\r',
		_ => return c,
	}
}