use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums_mut: Vec<i32> = nums;
        nums_mut.sort_unstable();
        let len: usize = nums_mut.len();
        let mut result: i32 = 0;
        for i in 0..(len >> 1) {
            let t: i32 = nums_mut[i] + nums_mut[len - 1 - i];
            if t > result {
                result = t;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
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

    if 2 > ret || ret & 1 == 1 {
        println!("Require even parameters more than 0.");
        return;
    }

    println!(
        "Minimize maximum pair sum: {}",
        Solution::min_pair_sum(nums)
    );
}
