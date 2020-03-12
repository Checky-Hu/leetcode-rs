use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn longest_arith_seq_length(a: Vec<i32>) -> i32 {
        let len: usize = a.len();
        if len < 3 {
            return len as i32;
        }
        let mut dp: Vec<HashMap<i32, i32>> = vec![HashMap::new(); len];
        let mut result: i32 = 2;
        for i in 0..len {
            for j in 0..i {
                let t: i32 = a[i] - a[j];
                let count: i32 = if let Some(x) = dp[j].get(&t) {
                    *x + 1
                } else {
                    2
                };
                dp[i].insert(t, count);
                if count > result {
                    result = count;
                }
            }
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
        "Longest arithmetic sequence length: {}",
        Solution::longest_arith_seq_length(a)
    );
}
