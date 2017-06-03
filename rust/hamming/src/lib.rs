pub fn hamming_distance(s1: &str, s2: &str) -> Result<usize, &'static str> {
    if s1.len() != s2.len() {
        return Err("Unequal length")
    }
    Ok(s1.chars().zip(s2.chars()).filter(|v| v.0 != v.1).count())
}
