impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut min = 0;
        let mut max = 0;


        match x {
            0..=100000000 => {
                min = 0;
                max = 10000;
                },
            100000001..=400000000 => {
                min = 10001;
                max = 20000;
                },
            400000001..=900000000 => {
                min = 20001;
                max = 30000;
                },
            900000001..=1600000000 => {
                min = 30001;
                max = 40000;
                },
            _ => {
                min = 40001;
                max = 46341;
            },
        };

        for i in min..max {
            if x < i * i {
                return (i as i32 - 1);
            } else if x == i * i {
                return i as i32;
            } 
        }

        46340
    }
}