pub fn square_of_sum(n: i32) -> i32 {
    let s: i32 = (1..(n+1)).sum();
    s*s
}

pub fn sum_of_squares(n: i32) -> i32 {
    (1..(n+1)).map(|x: i32| x*x).sum()
}

pub fn difference(n: i32) -> i32 {
    square_of_sum(n) - sum_of_squares(n)
}
