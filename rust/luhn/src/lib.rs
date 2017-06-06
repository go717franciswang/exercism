pub fn is_valid(s: &str) -> bool {
    let s = s.replace(" ", "");
    if s.len() < 2 {
        return false
    }

    let mut sum = 0;
    for (i, c) in s.chars().enumerate() {
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
