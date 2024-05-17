pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let n = digits.len();

    for i in (0..n).rev() {
        if digits[i] < 9 {
            digits[i] = digits[i] + 1;
            return digits;
        }
    }

    let mut new = vec![0; n + 1];
    new[0] = 1;
    new
}

pub fn single_number(nums: Vec<i32>) -> i32 {
    for i in (0..nums.len()).step_by(2) {
        if i == nums.len() - 1 || nums[i] != nums[i + 1] {
            return nums[i];
        }
    }

    0
}

pub fn string(s: &String) {
    let a = s;
    for i in a.chars().enumerate() {}
}
