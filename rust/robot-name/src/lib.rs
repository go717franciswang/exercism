extern crate rand;

use rand::{thread_rng, Rng};

pub struct Robot { 
    name: String,
}

impl Robot {
    pub fn new() -> Robot { 
        Robot { name: Self::gen_new_name() }
    }

    fn gen_new_name() -> String {
        let mut s = String::from("");
        s.push(Self::gen_letter());
        s.push(Self::gen_letter());
        s.push(Self::gen_digit());
        s.push(Self::gen_digit());
        s.push(Self::gen_digit());
        s
    }

    fn gen_letter() -> char {
        let mut rng = thread_rng();
        ('A' as u8 + rng.gen_range(0, 26)) as char
    }

    fn gen_digit() -> char {
        let mut rng = thread_rng();
        ('0' as u8 + rng.gen_range(0, 10)) as char
    }

    pub fn name<'a>(&'a self) -> &'a str { 
        self.name.as_str()
    }

    pub fn reset_name(&mut self) { 
        self.name = Self::gen_new_name();
    }
}
