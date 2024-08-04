impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut word = s.trim_end();
        let mut temp = word.rsplit_once(" ").unwrap_or(("",word)).1.len();
        temp as i32
    }
}