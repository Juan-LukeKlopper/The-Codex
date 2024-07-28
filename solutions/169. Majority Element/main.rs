use std::collections::HashMap;


impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut ans = 1;
        for i in 0..nums.len() {
            match map.get(&nums[i]) {
                Some(value) => {
                if let Some(freq) = map.get_mut(&nums[i]) {
                    *freq = *freq + 1;
                    if *freq > ans {
                        ans = *freq;
                    }
                }
                },
                None => {
                    map.insert(nums[i], 1);
                },

            }
        }

        for (key, value) in &map {
            if *value == ans {
                return *key;
            }
        }

        return 0
    }
}