pub fn rotate(s: &str, n: u8) -> String {
    s.chars().map(|c| {
        match c {
            'a'...'z' => (((c as u8 - 'a' as u8) + n) % 26 + 'a' as u8) as char,
            'A'...'Z' => (((c as u8 - 'A' as u8) + n) % 26 + 'A' as u8) as char,
            _ => c
        }
    }).collect::<String>()
}

