fn is_multiple(n: &i32, v: &Vec<i32>) -> bool {
    for d in v.iter() {
        if n % d == 0 {
            return true;
        } else if d > n {
            return false;
        }
    }
    false
}

pub fn sum_of_multiples(n: i32, v: &Vec<i32>) -> i32 {
    (1..n).filter(|x| is_multiple(x,v)).sum()
}
