use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map: HashMap<char, (i32, i32)> = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        let mut lowest = 100000;

        for (index, value) in chars.iter().enumerate() {
            match map.get_mut(&value) {
                Some(freq) => {
                    *freq = (0,0);
                },
                None => {
                    map.insert(*value, (1, index as i32));
                },

            }
        }

        for value in map.values() {
            if value.0 == 1 {
                if value.1 < lowest {
                    lowest = value.1;
                }
            }
        }

        if lowest == 100000 {
            return -1;
        }

        lowest
    }
}