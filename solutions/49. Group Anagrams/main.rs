use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let length = strs.len();
        let mut map: HashMap<Vec<char>, Vec<i32>> = HashMap::new();
        let mut chars: Vec<char>;
        let mut anagrams: Vec<Vec<String>> = Vec::new();
        let mut arrays: Vec<String> = Vec::new();

        if length <= 1 {
            return vec!(strs);
        }

        for (index, value) in strs.iter().enumerate() {
            chars = value.chars().collect();
            chars.sort_unstable();
            match map.get(&chars) {
                Some(value) => {
                if let Some(indexes) = map.get_mut(&chars) {
                    indexes.push(index as i32);
                }
                },
                None => {
                    map.insert(chars, vec!(index as i32));
                },

            }
        }
        

        for (key, value) in &map {
            for j in value.iter() {
                arrays.push(strs[*j as usize].clone() );
            }
            anagrams.push(arrays.clone());
            arrays.clear();
        }

        anagrams
    }
}