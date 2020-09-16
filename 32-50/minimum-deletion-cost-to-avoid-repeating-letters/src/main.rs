use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_cost(s: String, cost: Vec<i32>) -> i32 {
        // (letter, cost)
        let mut prefix: (char, i32) = ('0', 0);
        let mut result: i32 = 0;
        for (i, c) in s.chars().enumerate() {
            if c == prefix.0 {
                if prefix.1 >= cost[i] {
                    result += cost[i];
                } else {
                    result += prefix.1;
                    prefix.1 = cost[i];
                }
            } else {
                prefix.0 = c;
                prefix.1 = cost[i];
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s: String = String::new();
    let mut cost: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                cost.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!("Minimum cost of deletion: {}", Solution::min_cost(s, cost));
}
