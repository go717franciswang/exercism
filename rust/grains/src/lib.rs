pub fn square(s: u32) -> u64 {
    assert!(s < 65 && s > 0, "Square must be between 1 and 64");
    2_u64.pow(s-1)
}

pub fn total() -> u64 {
    (1..65).map(square).sum()
}
