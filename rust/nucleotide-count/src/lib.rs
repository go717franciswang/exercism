use std::collections::HashMap;

pub fn nucleotide_counts(seq: &str) -> Result<HashMap<char, usize>, &'static str> {
    let mut rs: HashMap<char, usize> = [
        ('A', 0),
        ('G', 0),
        ('T', 0),
        ('C', 0)].iter().cloned().collect();
    for c in seq.chars() {
        if rs.contains_key(&c) {
            let v = rs.get(&c).unwrap()+1;
            rs.insert(c, v);
        } else {
            return Err("Bad DNA");
        }
    }
    Ok(rs)
}

pub fn count(c: char, seq: &str) -> Result<usize, &'static str> {
    match nucleotide_counts(seq) {
        Ok(map) => match map.get(&c) {
            Some(v) => Ok(*v),
            None => Err("Bad DNA"),
        },
        Err(e) => Err(e)
    }
}

