pub fn anagrams_for<'a>(s: &str, inputs: &[&'a str]) -> Vec<&'a str> {
    inputs.iter().cloned().filter(|s2| eq(s, s2)).collect::<Vec<&str>>()
}

fn eq(s1: &str, s2: &str) -> bool {
    let mut set1 = s1.to_lowercase().chars().collect::<Vec<char>>();
    let mut set2 = s2.to_lowercase().chars().collect::<Vec<char>>();
    set1.sort();
    set2.sort();
    set1 == set2 && s1.to_lowercase() != s2.to_lowercase()
}
