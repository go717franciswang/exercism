pub fn abbreviate(s: &str) -> String {
    let mut abbr: String = String::new();
    let mut prev_char: char = ' ';
    for c in s.chars() {
        if (prev_char.is_lowercase() && c.is_uppercase()) || (!prev_char.is_alphanumeric() && c.is_alphanumeric()) {
            abbr.push(c);
        }
        prev_char = c;
    }
    abbr.to_uppercase()
}
