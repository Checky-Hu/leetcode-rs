use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let len: i32 = nums.len() as i32;
        let mut result: i32 = 0;
        for i in 0..32 {
            let mut z_count: i32 = 0;
            for num in &nums {
                if num & (1 << i) == 0 {
                    z_count += 1;
                }
            }
            result += (len - z_count) * z_count;
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
            nums.push(number);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Total hamming distance: {}", Solution::total_hamming_distance(nums));
}
