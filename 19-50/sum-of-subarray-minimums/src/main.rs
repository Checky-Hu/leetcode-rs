use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn sum_subarray_mins(a: Vec<i32>) -> i32 {
        let len: usize = a.len();
        let mut result: i32 = 0;
        let mut stack: Vec<i32> = vec![-1];
        let mut dp: Vec<i32> = vec![0; len + 1];
        for i in 0..len {
            let mut index: i32 = 0;
            while let Some(x) = stack.last() {
                if *x != -1 && a[i] <= a[*x as usize] {
                    stack.pop();
                } else {
                    index = *x;
                    break;
                }
            }
            dp[i + 1] = (dp[(index + 1) as usize] + (i as i32 - index) * a[i]) % 1_000_000_007;
            stack.push(i as i32);
            result = (result + dp[i + 1]) % 1_000_000_007;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            a.push(n);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Sum of subarray minimums: {}",
        Solution::sum_subarray_mins(a)
    );
}
