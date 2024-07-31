impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut sorted = nums.clone();
        sorted.sort();
        sorted.dedup();
        let mut duplicate = false;

        if sorted.len() == nums.len() {
            duplicate = false; 
        } else {
            duplicate = true;
        }

        return duplicate
    }
}