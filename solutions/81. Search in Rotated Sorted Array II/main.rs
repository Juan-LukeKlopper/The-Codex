impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        
        let mut found = false;

        for i in 0..nums.len() {
            if nums[i] == target {
                found = true;
            }
        }

        found
    }
}