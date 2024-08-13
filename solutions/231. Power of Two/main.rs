impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        let mut is_power_of_two: bool;
        let mut power_of_two_values: Vec<i32> = Vec::new();

        if n <= 0 {
            return false;
        }


        for i in 0..31 {
            power_of_two_values.push(2_i32.pow(i));
        }

        if power_of_two_values.contains(&n) {
            return true;
        }
        
        false
    }
}