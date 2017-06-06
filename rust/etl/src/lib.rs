use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut rs: BTreeMap<char, i32> = BTreeMap::new();

    for (i, chars) in input.iter() {
        for c in chars.iter() {
            rs.insert((*c).to_lowercase().next().unwrap(), *i);
        }
    }
    rs
}
