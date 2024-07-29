use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32,i32> = HashMap::new();
        let mut top_k_array: Vec<i32> = Vec::new();
        let mut temp: Vec<(&i32, &i32)> = Vec::new();

        for (index, value) in nums.iter().enumerate() {
            if let Some(freq) = map.get_mut(&value) {
                *freq += 1;
            } else {
                map.insert(*value, 1);
            }
        }
        
        temp = map.iter().collect();

        temp.sort_by(|a,b| b.1.cmp(&a.1));

        for (i) in 0..k {
            top_k_array.push(*temp[i as usize].0);
        }

        top_k_array
    }
}
