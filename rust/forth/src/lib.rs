use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    definitions: HashMap<String, Vec<String>>
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth { stack: vec![], definitions: HashMap::new() }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let terms = input
            .split(|c: char| c.is_whitespace() || c == ' ' || c == '\u{0000}' || c == '\u{0001}')
            .collect::<Vec<&str>>();
        self.eval_terms(&terms)
    }

    fn eval_terms(&mut self, terms: &[&str]) -> ForthResult {
        if terms.is_empty() {
            return Ok(())
        }

        match terms.iter().position(|s| s == &":") {
            Some(i) => match terms[i..].iter().position(|s| s == &";") {
                Some(j) => self.eval_ops(&terms[..i])
                    .and(self.eval_definition(&terms[(i+1)..(i+j)]))
                    .and(self.eval_terms(&terms[(i+j+1)..])),
                None => Err(Error::InvalidWord)
            },
            None => self.eval_ops(&terms[..])
        }
    }

    fn eval_definition(&mut self, terms: &[&str]) -> ForthResult {
        if terms.len() < 2 {
            return Err(Error::InvalidWord)
        }

        let (name, body) = terms.split_first().unwrap();
        if name.parse::<i32>().is_ok() {
            return Err(Error::InvalidWord)
        }

        self.definitions.insert(name.to_uppercase(), body.iter().map(|s| s.to_string()).collect());
        Ok(())
    }

    fn eval_ops(&mut self, terms: &[&str]) -> ForthResult {
        for term in terms.iter() {
            let term = term.to_uppercase();
            let parse_result = term.parse::<i32>();
            if parse_result.is_ok() {
                self.stack.push(parse_result.unwrap());
                continue
            }

            if self.definitions.contains_key(&term) {
                let mut body = vec![];
                {
                    body.append(&mut self.definitions.get_mut(&term).unwrap().clone());
                }
                let result = self.eval_ops(body.iter()
                                           .map(|s| s.as_str())
                                           .collect::<Vec<&str>>()
                                           .as_slice());
                if result.is_err() {
                    return result
                }
                continue
            }

            let result = match term.as_str() {
                "+" => self.op_2_args(|a,b| vec![a + b]),
                "-" => self.op_2_args(|a,b| vec![a - b]),
                "*" => self.op_2_args(|a,b| vec![a * b]),
                "/" => self.op_divide(),
                "DUP" => self.op_1_arg(|a| vec![a, a]),
                "DROP" => self.op_1_arg(|_a| vec![]),
                "SWAP" => self.op_2_args(|a,b| vec![b, a]),
                "OVER" => self.op_2_args(|a,b| vec![a, b, a]),
                _ => Err(Error::UnknownWord)
            };

            if result.is_err() {
                return result
            }
        }

        Ok(())
    }

    fn op_2_args<F>(&mut self, f: F) -> Result<(), Error> where F: Fn(i32, i32) -> Vec<i32> {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow)
        }

        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        for x in f(a, b) {
            self.stack.push(x);
        }
        Ok(())
    }

    fn op_1_arg<F>(&mut self, f: F) -> Result<(), Error> where F: Fn(i32) -> Vec<i32> {
        if self.stack.len() < 1 {
            return Err(Error::StackUnderflow)
        }

        for x in f(self.stack.pop().unwrap()) {
            self.stack.push(x);
        }
        Ok(())
    }

    fn op_divide(&mut self) -> Result<(), Error> {
        match self.stack.last() {
            Some(&0) => Err(Error::DivisionByZero),
            _ => self.op_2_args(|a,b| vec![a / b])
        }
    }
}
