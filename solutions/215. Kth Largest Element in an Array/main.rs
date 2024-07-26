use std::convert::TryInto;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut sorted = Vec::new();
        let index: usize = (k - 1).try_into().unwrap();

        for i in 0..nums.len() {
            let index: usize = i.try_into().unwrap();
            sorted.push(nums[i]);
        }

        sorted.sort_unstable();
        sorted.reverse();

        sorted[index]
    }
}