use std::collections::HashMap;
use std::collections::BTreeSet;

#[derive(Debug)]
struct Equation {
    left: HashMap<char, u32>,
    right: HashMap<char, u32>,
    leading_tokens: BTreeSet<char>
}

impl Equation {
    fn new(puzzle: &str) -> Self {
        let sides = puzzle.split(" == ").collect::<Vec<&str>>();
        Equation {
            left: Self::extract_multipliers(sides[0]),
            right: Self::extract_multipliers(sides[1]),
            leading_tokens: Self::extract_leading_tokens(puzzle)
        }
    }

    fn extract_leading_tokens(puzzle: &str) -> BTreeSet<char> {
        puzzle.split(' ')
            .map(|term| term.chars().nth(0).unwrap())
            .filter(|c| c.is_alphabetic())
            .collect::<BTreeSet<char>>()
    }

    fn extract_multipliers(side: &str) -> HashMap<char, u32> {
        let mut m: HashMap<char, u32> = HashMap::new();
        for term in side.split(" + ") {
            for (i, c) in term.chars().rev().enumerate() {
                *m.entry(c).or_insert(0) += 10_u32.pow(i as u32);
            }
        }
        m
    }

    fn compute(multipliers: &HashMap<char, u32>, token_val: &HashMap<char, u8>) -> u32 {
        let mut v: u32 = 0;
        for (c, m) in multipliers.iter() {
            v += m * (*token_val.get(c).unwrap() as u32);
        }
        v
    }

    fn is_valid(&self, token_val: &HashMap<char, u8>) -> bool {
        Self::compute(&self.left, token_val) == Self::compute(&self.right, token_val) 
            && self.leading_tokens.iter().all(|c| token_val.get(c).unwrap() > &0)
    }
}

pub struct DigitPermutationIterator {
    width: u8,
    n: u64,
    max: u64
}

impl DigitPermutationIterator {
    pub fn new(w: u8) -> Self {
        Self { 
            width: w, 
            n: 0,
            max: 10_u64.pow((w as u32)+1)-1
        }
    }

    fn are_digits_distinct(&self) -> bool {
        let mut n = self.n;
        let mut mask = 0_u16;
        for _ in 0..(self.width) {
            let d = n % 10;
            if mask & (1 << d) != 0 {
                return false
            }
            mask |= 1 << d;
            n /= 10;
        }
        true
    }
}

impl Iterator for DigitPermutationIterator {
    type Item = Digits;

    fn next(&mut self) -> Option<Digits> {
        self.n += 1;
        while !self.are_digits_distinct() {
            if self.n > self.max {
                return None
            }
            self.n += 1;
        }
        return Some(Digits { n: self.n, w: self.width })
    }
}

#[derive(PartialEq, Debug)]
pub struct Digits { pub n: u64, pub w: u8 }
impl IntoIterator for Digits {
    type Item = u8;
    type IntoIter = DigitIter;

    fn into_iter(self) -> DigitIter {
        DigitIter { n: self.n, w: self.w }
    }
}

pub struct DigitIter { pub n: u64, pub w: u8 }
impl Iterator for DigitIter {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.w == 0 {
            None
        } else {
            let v = self.n % 10;
            self.n /= 10;
            self.w -= 1;
            Some(v as u8)
        }
    }
}

pub fn solve(puzzle: &str) -> Option<HashMap<char, u8>> {
    let mut tokens = puzzle.chars().filter(|c| c.is_alphabetic()).collect::<Vec<char>>();
    tokens.sort();
    tokens.dedup();

    let mut token_val: HashMap<char, u8> = HashMap::new();
    let equation = Equation::new(puzzle);
    for perm in DigitPermutationIterator::new(tokens.len() as u8) {
        for (token, val) in tokens.iter().zip(perm) {
            token_val.insert(*token, val);
        }

        if equation.is_valid(&token_val) {
            return Some(token_val);
        }
    }

    None
}
