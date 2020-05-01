use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = vec![0, i32::min_value(), i32::min_value()];
        for num in nums.iter() {
            let mut tmp: Vec<i32> = Vec::with_capacity(3);
            for i in 0..3 {
                let index: i32 = if i < *num {
                    (3 + (i - *num) % 3) % 3
                } else {
                    (i - *num) % 3
                };
                let t: i32 = dp[index as usize] + *num;
                tmp.push(if dp[i as usize] > t {
                    dp[i as usize]
                } else {
                    t
                });
            }
            dp = tmp;
        }
        dp[0]
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
        "Maximum sum divided by 3: {}",
        Solution::max_sum_div_three(nums)
    );
}
