pub fn lsp(s: &str, l: usize) -> Result<u32, &'static str> {
    if s.len() < l {
        return Err("Span is longer than number")
    }

    let mut digits: Vec<u32> = vec![];
    for c in s.chars() {
        match c.to_digit(10) {
            Some(n) => digits.push(n),
            None => return Err("Invalid input")
        }
    }

    let mut product = digits[0..l].iter().fold(1, |x,p| x*p);
    let mut max = product;
    for i in l..digits.len() {
        if digits[i-l] == 0 {
            product = digits[(i-l+1)..(i+1)].iter().fold(1, |x,p| x*p)
        } else {
            product = product / digits[i-l] * digits[i]
        }
        max = std::cmp::max(max, product)
    }
    Ok(max)
}
