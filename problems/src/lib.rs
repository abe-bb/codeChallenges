/*
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.
*/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums: Vec<(usize, &i32)> = nums.iter().enumerate().collect();
    nums.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut result: Vec<i32> = Vec::new();

    let mut lower = 0;
    let mut upper = nums.len() - 1;

    loop {
        let test = nums[lower].1 + nums[upper].1;
        if test == target {
            result.push(nums[lower].0.try_into().unwrap());
            result.push(nums[upper].0.try_into().unwrap());
            break;
        } else if test < target {
            lower += 1;
        } else {
            upper -= 1;
        }

        assert!(lower < upper);
    }

    result
}
