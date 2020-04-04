use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut temp: Vec<i32> = nums.clone();
        temp.sort();
        let mut status: Vec<i32> = vec![0; 101];
        // (number, sum)
        let mut prefix: (i32, i32) = (-1, 0);
        for n in temp.iter() {
            if prefix.0 != *n {
                status[*n as usize] = prefix.1;
                prefix.0 = *n;
            }
            prefix.1 += 1;
        }
        let mut result: Vec<i32> = Vec::new();
        for n in nums.iter() {
            result.push(status[*n as usize]);
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

    let result: Vec<i32> = Solution::smaller_numbers_than_current(nums);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
