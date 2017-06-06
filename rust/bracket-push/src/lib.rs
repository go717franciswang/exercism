pub struct Brackets {
    expression: String
}

impl<'a> From<&'a str> for Brackets {
    fn from(e: &'a str) -> Brackets {
        Brackets { expression: e.to_string() }
    }
}

impl Brackets {
    pub fn are_balanced(&self) -> bool {
        let mut stack: Vec<char> = vec![];
        for c in self.expression.chars() {
            match c {
                '{'|'['|'(' => stack.push(c),
                '}' => match stack.pop() { Some('{') => continue, _ => return false },
                ']' => match stack.pop() { Some('[') => continue, _ => return false },
                ')' => match stack.pop() { Some('(') => continue, _ => return false },
                _ => continue
            }
        }
        stack.is_empty()
    }
}
