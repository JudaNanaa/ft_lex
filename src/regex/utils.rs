pub trait RemoveVecElement<T>
where
    T: PartialEq,
{
    fn remove_element(&mut self, element: &T) -> Option<T>;
    fn push_unique(&mut self, element: T);
}

impl<T> RemoveVecElement<T> for Vec<T>
where
    T: PartialEq,
{
    fn remove_element(&mut self, element: &T) -> Option<T> {
        self.iter()
            .position(|e| e == element)
            .map(|index| self.remove(index))
    }

    fn push_unique(&mut self, element: T) {
        if !self.contains(&element) {
            self.push(element);
        }
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
