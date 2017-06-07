pub trait Luhn: ToString {
    fn valid_luhn(&self) -> bool {
        let s = self.to_string().replace(" ", "");
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

impl<'a> Luhn for &'a str {}
impl Luhn for String {}
impl Luhn for u8 {}
impl Luhn for u16 {}
impl Luhn for u32 {}
impl Luhn for u64 {}
impl Luhn for usize {}
