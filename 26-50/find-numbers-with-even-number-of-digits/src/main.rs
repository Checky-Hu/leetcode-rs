use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        for n in &nums {
            let mut mul: i32 = 10;
            let mut bit: i32 = 1;
            while mul <= *n {
                mul *= 10;
                bit += 1;
            }
            if bit & 1 == 0 {
                result += 1;
            }
        }
        result
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
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Numbers with even number of digits: {}",
        Solution::find_numbers(nums)
    );
}
