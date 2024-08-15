impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut chars: Vec<char> = s.chars().collect();
        let mut total = 0;

        let minus_one = ['V','X'];
        let minus_ten = ['L','C'];
        let minus_hun = ['D','M'];

        for i in 0..chars.len() {

            match chars[i] {
                'I' => {
                    if i == chars.len()-1 {
                        total += 1
                    } else if minus_one.contains(&chars[i + 1])  {
                        total += -1
                    } else {
                        total += 1
                    }
                },
                'V' => {
                    total += 5
                },
                'X' => {
                    if i == chars.len()-1 {
                        total += 10
                    } else if minus_ten.contains(&chars[i + 1])  {
                        total += -10
                    } else {
                        total += 10
                    }
                },
                'L' => {
                    total += 50
                },
                'C' => {
                    if i == chars.len()-1 {
                        total += 100
                    } else if minus_hun.contains(&chars[i + 1])  {
                        total += -100
                    } else {
                        total += 100
                    }
                },
                'D' => {
                    total += 500
                },
                _ => {
                    total += 1000
                },
            }
        }

        total
    }
}