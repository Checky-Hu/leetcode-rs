use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let mut temp: Vec<i32> = nums;
        temp.sort_by(|a, b| b.cmp(a));
        let mut sum: i32 = 0;
        let mut record: Vec<i32> = Vec::new();
        for t in temp.iter() {
            sum += *t;
            record.push(sum);
        }
        let mut result: Vec<i32> = Vec::new();
        for (i, r) in record.iter().enumerate() {
            result.push(temp[i]);
            if *r * 2 > sum {
                break;
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

    let result: Vec<i32> = Solution::min_subsequence(nums);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
