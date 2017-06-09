use std::collections::BTreeSet;

pub struct Robot { 
    name: &str,
}
let static mut names_used: BTreeSet<String> = BTreeSet::new();

impl Robot {
    pub fn new() -> Robot { 
        let r = Robot { }
    }

    fn gen_new_name() -> &str {

    }

    pub fn name<'a>(&'a self) -> &'a str { 
    }

    pub fn reset_name(&mut self) { 
    }
}
