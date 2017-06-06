use std::ascii::AsciiExt;

// pub fn encode(s: &str) -> String {
//     let mut rs = String::new();
//     for c in s.to_lowercase().chars() {
//         if !c.is_alphanumeric() || !c.is_ascii() {
//             continue;
//         }
// 
//         if rs.len() % 6 == 5 {
//             rs.push(' ');
//         }
// 
//         if c.is_alphabetic() {
//             rs.push((25_u8 + ('a' as u8)*2- c as u8) as char);
//         } else {
//             rs.push(c);
//         }
//     }
//     rs
// }

pub fn encode(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .filter(|c| c.is_ascii() && c.is_alphanumeric())
        .map(|c| 
            if c.is_alphabetic() {
                (25_u8 + ('a' as u8)*2 - c as u8) as char
            } else {
                c
            })
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|chars| chars.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn decode(s: &str) -> String {
    encode(&s.replace(' ', "")).replace(' ', "")
}
