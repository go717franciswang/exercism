pub fn raindrops(n: i32) -> String {
    let mut s = "".to_string();
    if n % 3 == 0 {
        s += &"Pling";
    }
    if n % 5 == 0 {
        s += &"Plang";
    }
    if n % 7 == 0 {
        s += &"Plong";
    }
    if s.len() == 0 {
        s += &format!("{}", n);
    }
    s
}
