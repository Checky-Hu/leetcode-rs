use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let mut vec: Vec<i32> = Vec::with_capacity((n + 1) as usize);
        vec.push(0);
        let mut sum: i32 = 0;
        for num in nums.iter() {
            sum += *num;
            vec.push(sum);
        }
        let mut all: Vec<i32> = Vec::with_capacity((n * (n - 1) / 2) as usize);
        for i in 0..n {
            for j in (i + 1)..=n {
                vec[j as usize] -= vec[i as usize];
                all.push(vec[j as usize]);
            }
        }
        all.sort();
        let mut result: i32 = 0;
        let modulo: i32 = 1_000_000_007;
        for i in left..=right {
            result = (result + all[i as usize - 1]) % modulo;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut left: i32 = 0;
    let mut right: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            2 => left = i32::from_str(&arg).expect("Error parse."),
            3 => right = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(t);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 3 parameters.");
        return;
    }

    println!("Range sum: {}", Solution::range_sum(nums, n, left, right));
}
