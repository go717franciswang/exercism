pub struct Luhn {
    s: String
}

impl<T> From<T> for Luhn where T: ToString {
    fn from(n: T) -> Luhn {
        Luhn { s: n.to_string() }
    }
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let s = self.s.replace(" ", "");
        if s.len() < 2 {
            return false
        }

        let mut sum = 0;
        for (i, c) in s.char_indices() {
            let n = (c as i16) - ('0' as i16);
            if n < 0 || n > 9 {
                return false
            }

            if (s.len() - i) % 2 == 0 {
                if n * 2 > 9 {
                    sum += n * 2 - 9
                } else {
                    sum += n * 2
                }
            } else {
                sum += n
            }
        }

        sum % 10 == 0
    }
}

