impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut x: Vec<char> = s.chars().collect();
        let mut y: Vec<char> = t.chars().collect();
        let mut anagram = false;

        x.sort_unstable();
        y.sort_unstable();

        if x == y {
            anagram = true
        }

        return anagram
    }
}