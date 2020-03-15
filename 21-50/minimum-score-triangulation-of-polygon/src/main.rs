use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn helper(a: &[i32], s: usize, e: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        if s + 1 == e {
            return 0;
        }
        if memo[s][e] > 0 {
            return memo[s][e];
        }
        memo[s][e] = i32::max_value();
        for i in (s + 1)..e {
            let t: i32 = a[s] * a[i] * a[e]
                + Solution::helper(a, s, i, memo)
                + Solution::helper(a, i, e, memo);
            if t < memo[s][e] {
                memo[s][e] = t;
            }
        }
        memo[s][e]
    }

    pub fn min_score_triangulation(a: Vec<i32>) -> i32 {
        let len: usize = a.len();
        let mut memo: Vec<Vec<i32>> = vec![vec![0; len]; len];
        Solution::helper(&a, 0, len - 1, &mut memo)
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
        "Minimum score triangulation: {}",
        Solution::min_score_triangulation(a)
    );
}
