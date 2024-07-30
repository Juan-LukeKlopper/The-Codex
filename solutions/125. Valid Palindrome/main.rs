impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut tf = true;
        let mut alpha: String = s.to_lowercase().chars().filter(|c| !c.is_whitespace()).filter(|c| c.is_alphanumeric()).collect();
        let mut reverse_alpha: String = alpha.chars().rev().collect();

        if alpha == reverse_alpha {
            tf = true;
        } else {
            tf = false;
        }

        tf
    }
}