impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut new_digits = digits.clone();
        let length = new_digits.len();
        let mut temp_value;
        let mut position;


        let mut buffer: u8 = 1;

        for (index, value) in digits.iter().rev().enumerate() {
            if buffer == 0 {
                break;
            }
            position = length - index - 1;
            new_digits.remove(position);
            temp_value = (value + 1) % 10;
            new_digits.insert(position, temp_value);

            if *value != 9 {
                buffer = 0
            }

            if position == 0 && buffer == 1 {
                new_digits.insert(0, 1);
            }
        }

        new_digits
    }
}