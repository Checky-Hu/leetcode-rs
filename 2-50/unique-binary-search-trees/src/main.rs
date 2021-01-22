use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn num_trees_loop(n: usize, vec: &mut Vec<i32>) -> i32 {
        if vec[n] != -1 {
            return vec[n];
        }
        let mut result: i32 = 0;
        for i in 1..=n {
            result += Solution::num_trees_loop(i - 1, vec) * Solution::num_trees_loop(n - i, vec);
        }
        vec[n] = result;
        result
    }

    pub fn num_trees(n: i32) -> i32 {
        let usize_n: usize = n as usize;
        let mut vec: Vec<i32> = vec![-1; usize_n + 1];
        vec[0] = 1;
        vec[1] = 1;
        Solution::num_trees_loop(usize_n, &mut vec)
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Number of trees: {}", Solution::num_trees(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
