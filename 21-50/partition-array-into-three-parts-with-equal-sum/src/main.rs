use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        let mut sum: i32 = 0;
        for n in &a {
            sum += n;
        }
        if sum % 3 != 0 {
            return false
        } else {
            let target: i32 = sum / 3;
            let mut count: i32 = 0;
            let mut cur_sum: i32 = 0;
            for n in &a {
                cur_sum += n;
                if cur_sum == target {
                    cur_sum = 0;
                    count += 1;
                }
            }
            count == 3
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                a.push(n);
            },
        }
    }

    if 3 > ret {
        println!("Require at least three parameters.");
        return;
    }

    println!("Partition 3: {}", Solution::can_three_parts_equal_sum(a));
}

