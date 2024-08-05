use std::collections::HashMap;


impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut vec_note: Vec<char> = ransom_note.chars().collect();
        let mut vec_magazine: Vec<char> = magazine.chars().collect();

        if vec_note.len() > vec_magazine.len() {
            return false;
        }

        for (value) in vec_magazine.iter() {
            match map.get_mut(&value) {
                Some(item) => {
                    *item = *item + 1;
                },
                None => {
                    map.insert(*value, 1);
                },

            }
        }


        for (value) in vec_note.iter() {
            match map.get_mut(&value) {
                Some(item) => {
                    if *item <= 0 {
                        return false;
                    } else {
                        *item = *item - 1;
                    }
                },
                None => {
                    return false;
                },

            }
        }


        true
        
    }
}