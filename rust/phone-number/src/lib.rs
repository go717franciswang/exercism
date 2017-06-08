pub fn number(s: &str) -> Option<String> {
    let mut nums: String = s.chars().filter(|c| c.is_numeric()).collect::<String>();
    if nums.len() == 11 {
        if nums.starts_with("1") {
            nums = nums[1..].to_string()
        } else {
            return None
        }
    } else if nums.len() != 10 {
        return None
    }

    Some(nums)
}

pub fn area_code(s: &str) -> Option<String> {
    number(s).map(|nums| nums[..3].to_string())
}

pub fn pretty_print(s: &str) -> String {
    match number(s) {
        Some(nums) => format!("({}) {}-{}", &nums[..3], &nums[3..6], &nums[6..]),
        None => String::from("invalid")
    }
}
