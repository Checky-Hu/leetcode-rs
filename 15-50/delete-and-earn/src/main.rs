use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let len: usize = 10000;
        let mut sums: Vec<i32> = vec![0; len + 1];
        for num in &nums {
            sums[*num as usize] += *num;
        }
        for i in 2..=len {
            let t1: i32 = sums[i - 2] + sums[i];
            let t2: i32 = sums[i - 1];
            sums[i] = if t1 > t2 { t1 } else { t2 };
        }
        sums[len]
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Max earn: {}", Solution::delete_and_earn(nums));
}
