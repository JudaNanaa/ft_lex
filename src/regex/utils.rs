pub trait RemoveVecElement<T> 
    where T: PartialEq
{
    fn remove_element(&mut self, element: &T) -> Option<T>;
}

impl<T> RemoveVecElement<T> for Vec<T> 
    where T: PartialEq
{
    fn remove_element(&mut self, element: &T) -> Option<T>{

        self.iter()
            .position(|_element| _element == element)
            .map(|index| self.remove(index))
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
