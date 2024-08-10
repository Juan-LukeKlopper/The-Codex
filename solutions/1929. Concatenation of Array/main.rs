impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let nums2 = nums.clone();

        let ans: Vec<i32> = [nums, nums2].concat();
        ans
    }
}