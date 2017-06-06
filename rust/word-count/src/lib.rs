use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {
    let mut rs: HashMap<String, u32> = HashMap::new();
    for word in s.to_lowercase().split(|c: char| !c.is_alphanumeric()) {
        if word.is_empty() {
            continue
        }

        let w = word.to_string();
        if rs.contains_key(&w) {
            let n = *rs.get(&w).unwrap();
            rs.insert(w, n+1);
        } else {
            rs.insert(w, 1);
        }
    }
    rs
}
