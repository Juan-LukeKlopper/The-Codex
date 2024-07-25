use std::convert::TryInto;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut index: usize = 0;
        let mut pre_num: i32 = 0;
        let mut freq = 0;

        for i in 0..nums.len() {
            let number = nums[index];
            if index == 0 {
                pre_num = number;
                freq = freq + 1;
                index = index + 1;
            } else if number == pre_num {
                if freq >= 2 {
                    nums.remove(index);
                } else {
                    freq = freq + 1;
                    index = index + 1;
                }
            } else {
                pre_num = number;
                freq = 1;
                index = index + 1;
            }
        }

        nums.len().try_into().unwrap()
    }
}