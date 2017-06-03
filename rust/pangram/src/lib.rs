pub fn is_pangram(sentence: &str) -> bool {
    let mut found_letter = [false; 26];
    for c in sentence.to_lowercase().chars() {
        let idx = (c as i16) - ('a' as i16);
        if idx >= 0 && idx < 26 {
            found_letter[idx as usize] = true;
        }
    }
    found_letter.iter().all(|x| *x)
}
