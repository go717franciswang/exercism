pub fn encode(s: &str) -> String {
    if s.is_empty() {
        return String::new()
    }

    let mut iter = s.chars().peekable();
    let mut counter = 0;
    let mut rs = "".to_string();
    loop {
        match iter.next() {
            Some(c) => {
                match iter.peek() {
                    Some(c2) if c==*c2 => counter += 1,
                    _ => {
                        if counter > 0 {
                            rs += &format!("{}{}", counter+1, c); 
                            counter = 0
                        } else {
                            rs.push(c)
                        }
                    },
                }
            },
            None => break
        }
    }
    rs
}

pub fn decode(s: &str) -> String {
    let mut n = 0_usize;
    let mut rs = String::new();
    for c in s.chars() {
        let c = c.to_string();
        match c.parse::<usize>() {
            Ok(d) => n = n*10+d,
            _ => {
                rs += &(&c).repeat(*[1,n].iter().max().unwrap());
                n = 0;
            }
        }
    }
    rs
}
