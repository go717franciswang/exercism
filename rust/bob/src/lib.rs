pub fn reply(s: &str) -> &str {
    if s.len() == 0 {
        return "Fine. Be that way!";
    }

    if s.ends_with('?') {
        return "Sure.";
    }

    for c in s.chars() {
        if (c as u8) >= 97 && (c as u8) <= 122 {
            return "Whatever.";
        }
    }

    "Whoa, chill out!"
}
