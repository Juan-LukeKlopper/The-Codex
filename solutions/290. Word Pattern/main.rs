use std::collections::HashMap;


impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut map: HashMap<char, &str> = HashMap::new();
        let s_vec: Vec<&str> = s.split(' ').collect();
        let mut values: Vec<&str> = Vec::new();

        if pattern.len() != s_vec.len() {
            return false;
        }

        for (index, value) in pattern.chars().enumerate() {
            match map.get_mut(&value) {
                Some(pairing) => {
                    if *pairing != s_vec[index] {
                        return false;
                    }
                },
                None => {
                    map.insert(value, s_vec[index]);
                },
            }
        }

        for val in map.values() {
            if values.contains(val) {
                return false;
            } else {
                values.push(val);
            }
        }

        true
    }
}