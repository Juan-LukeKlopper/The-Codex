use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map_s: HashMap<char, Vec<i32>> = HashMap::new();
        let mut map_t: HashMap<char, Vec<i32>> = HashMap::new();

        if t.len() != s.len() {
            return false;
        }

        for (index, value) in s.chars().enumerate() {
            match map_s.get_mut(&value) {
                Some(indexes) => {
                    indexes.push(index as i32);
                },
                None => {
                    map_s.insert(value, vec!(index as i32));
                },
            }
        }

        for (index, value) in t.chars().enumerate() {
            match map_t.get_mut(&value) {
                Some(indexes) => {
                    indexes.push(index as i32);
                },
                None => {
                    map_t.insert(value, vec![index as i32]);
                },
            }
        }

        let mut s_vec: Vec<Vec<i32>> = map_s.into_values().collect();
        let mut t_vec: Vec<Vec<i32>> = map_t.into_values().collect();

        s_vec.sort_unstable();
        t_vec.sort_unstable();

        if s_vec != t_vec {
            return false;
        }

        true
    }
}