pub struct Roman {
    n: u16
}

impl Roman {
    pub fn from(n: u16) -> Roman {
        Roman { n: n }
    }
}

impl ToString for Roman {
    fn to_string(&self) -> String {
        let mut rs = String::new();

        rs += match self.n / 1000 {
            1 => "M",
            2 => "MM",
            3 => "MMM",
            _ => ""
        };

        rs += match self.n % 1000 / 100 {
            1 => "C",
            2 => "CC",
            3 => "CCC",
            4 => "CD",
            5 => "D",
            6 => "DC",
            7 => "DCC",
            8 => "DCCC",
            9 => "CM",
            _ => ""
        };


        rs += match self.n % 100 / 10 {
            1 => "X",
            2 => "XX",
            3 => "XXX",
            4 => "XL",
            5 => "L",
            6 => "LX",
            7 => "LXX",
            8 => "LXXX",
            9 => "XC",
            _ => ""
        };

        rs += match self.n % 10 {
            1 => "I",
            2 => "II",
            3 => "III",
            4 => "IV",
            5 => "V",
            6 => "VI",
            7 => "VII",
            8 => "VIII",
            9 => "IX",
            _ => ""
        };

        rs
    }
}
