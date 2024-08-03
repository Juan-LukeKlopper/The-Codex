use std::convert::TryInto;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut index: i32 = -1;

        if needle.len() > haystack.len() {
            return index
        } else if needle.len() == haystack.len() {
            if needle == haystack {
                return 0
            } 
        } else {
            for i in needle.len()..haystack.len()+1 {
                let start = i - needle.len();
                let chunk = haystack.get(start..i).unwrap();
                if chunk == needle {
                    index = start.try_into().unwrap();
                    break
                }
            }
        }
        return index
    }
}