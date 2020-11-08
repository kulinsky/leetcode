pub fn check_possibility(nums: Vec<i32>) -> bool {
    let mut pointer = None;
    for idx in 0..(nums.len() - 1) {
        if nums[idx] > nums[idx + 1] {
            match pointer {
                None => pointer = Some(idx),
                Some(_) => return false,
            }
        }
    }
    return match pointer {
        None => true,
        Some(0) => true,
        Some(i) => {
            return if i == nums.len() - 2 || nums[i - 1] <= nums[i + 1] || nums[i] <= nums[i + 2] {
                true
            } else {
                false
            }
        }
    };
}
