use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn helper(a: &[i32], k: usize, left: usize, right: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        if memo[left][right] < 0 {
            if left + k > right {
                let mut max: i32 = 0;
                for i in left..=right {
                    if a[i] > max {
                        max = a[i];
                    }
                }
                memo[left][right] = max * (right - left + 1) as i32;
            } else {
                let mut result: i32 = 0;
                for i in left..right {
                    let t: i32 = Solution::helper(a, k, left, i, memo)
                        + Solution::helper(a, k, i + 1, right, memo);
                    if t > result {
                        result = t;
                    }
                }
                memo[left][right] = result;
            }
        }
        memo[left][right]
    }

    pub fn max_sum_after_partitioning(a: Vec<i32>, k: i32) -> i32 {
        let len: usize = a.len();
        let mut memo: Vec<Vec<i32>> = vec![vec![-1; len]; len];
        Solution::helper(&a, k as usize, 0, len - 1, &mut memo)
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                a.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Max sum fater partitioning: {}",
        Solution::max_sum_after_partitioning(a, k)
    );
}
