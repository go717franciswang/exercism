pub struct WordProblem {
    command: String
}

#[derive(Clone)]
enum Op {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl WordProblem {
    pub fn new(command: &str) -> Self {
        WordProblem { command: command.to_string() }
    }

    pub fn answer(&self) -> Result<i32, &'static str> {
        let mut command = self.command.to_string();
        if command.starts_with("What is ") {
            command.drain(..("What is ".len()));
        } else {
            return Err("Non math question")
        }

        let mut ans: i32 = Self::extract_num(&mut command).unwrap();
        loop {
            if command.is_empty() {
                break;
            }
            println!("{} '{}'", ans, command);

            let op = match Self::extract_op(&mut command) {
                Some(op) => op,
                None => return Err("Unknown command")
            };
            let n = match Self::extract_num(&mut command) {
                Some(n) => n,
                None => return Err("Can't find number")
            };
            
            match op {
                Op::Add => ans += n,
                Op::Subtract => ans -= n,
                Op::Multiply => ans *= n,
                Op::Divide => ans /= n,
            }
        }

        Ok(ans)
    }

    fn extract_num(command: &mut String) -> Option<i32> {
        let mut n: i32 = 0;
        let mut sign: i32 = 1;
        let mut len: usize = 0;
        for (i, c) in command.char_indices() {
            match c {
                '-' if i == 0 => sign = -1,
                '0'...'9' => n = n*10+(c as i32 - '0' as i32),
                _ if i == 0 => return None,
                _ => { len = i; break }
            }
        }
        command.drain(..(len+1));

        Some(n*sign)
    }

    fn extract_op(command: &mut String) -> Option<Op> {
        let op_map = [
            ("plus ", Op::Add),
            ("minus ", Op::Subtract),
            ("multiplied by ", Op::Multiply),
            ("divided by ", Op::Divide)
        ];

        for &(ref s, ref op) in op_map.iter() {
            if command.starts_with(&s.to_string()) {
                command.drain(..(s.len()));
                return Some(op.clone())
            }
        }

        return None
    }
}
