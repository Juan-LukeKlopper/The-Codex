impl Solution {
    pub fn is_palindrome(x: i32) -> bool {

        if x < 0 {
            return false;
        }


        let digits: Vec<i32> = x.to_string()
                                    .chars()
                                    .map(|c| c.to_digit(10).unwrap() as i32)
                                    .collect();

        let length = digits.len();


        for (index, value) in digits.iter().enumerate() {
            if *value != digits[(length - index - 1)] {
                return false;
            }
        }

        true
    }
}