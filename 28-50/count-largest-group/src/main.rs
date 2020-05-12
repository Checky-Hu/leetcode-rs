use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut counts: Vec<i32> = vec![0; 36];
        for i in 1..=n {
            let mut sum: i32 = 0;
            let mut tmp: i32 = i;
            while tmp > 0 {
                sum += tmp % 10;
                tmp /= 10;
            }
            counts[sum as usize - 1] += 1;
        }
        let mut max: i32 = 0;
        let mut result: i32 = -1;
        for v in counts.iter() {
            if *v > max {
                max = *v;
                result = 1;
            } else if *v == max {
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Largest group number: {}", Solution::count_largest_group(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
