use std::collections::BTreeMap;

pub struct Info {
    map: BTreeMap<&'static str, &'static str>
}

pub fn parse(pairs: Vec<(&'static str, &'static str)>) -> Info {
    Info { map: pairs.iter().cloned().collect::<BTreeMap<&'static str, &'static str>>() }
}

impl Info {
    pub fn name_for(&self, codon: &str) -> Result<&'static str, &'static str> {
        if codon.len() != 3 {
            return Err("Wrong length")
        }

        for decompressed in Self::decompress(codon) {
            let found = self.map.get(decompressed.as_str());
            if found.is_some() {
                return Ok(found.unwrap())
            }
        }

        Err("Not found")
    }

    pub fn expand(c: char) -> Vec<char> {
        match c {
            'N' => vec!['U', 'C', 'A', 'G'],
            'M' => vec!['C', 'A'],
            'R' => vec!['A', 'G'],
            'Y' => vec!['U', 'C'],
            'H' => vec!['U', 'C', 'A'],
            c => vec![c]
        }
    }

    pub fn decompress(codon: &str) -> Vec<String> {
        let chars = codon.chars().collect::<Vec<char>>();
        let mut rs: Vec<String> = vec![];

        for c0 in Self::expand(chars[0]) {
            for c1 in Self::expand(chars[1]) {
                for c2 in Self::expand(chars[2]) {
                    rs.push(format!("{}{}{}", c0, c1, c2));
                }
            }
        }

        rs
    }
}
