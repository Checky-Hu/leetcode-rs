use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let len: usize = s.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; 26]; len];
        for (i, u) in s.as_bytes().iter().enumerate() {
            if i == 0 {
                dp[0][*u as usize - 97] += 1;
            } else {
                for j in 0..26 {
                    if j + 97 == *u as usize {
                        dp[i][j] = dp[i - 1][j] + 1;
                    } else {
                        dp[i][j] = dp[i - 1][j];
                    }
                }
            }
        }
        let mut result: Vec<bool> = Vec::new();
        for query in queries.iter() {
            if query[0] == query[1] || query[1] - query[0] < query[2] {
                result.push(true);
                continue;
            }
            let mut count: i32 = 0;
            for i in 0..26 {
                count += if query[0] == 0 {
                    dp[query[1] as usize][i]
                } else {
                    dp[query[1] as usize][i] - dp[query[0] as usize - 1][i]
                } & 1;
            }
            if count / 2 > query[2] {
                result.push(false);
            } else {
                result.push(true);
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s: String = String::new();
    let mut queries: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(n);
                if tmp.len() == 3 {
                    queries.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<bool> = Solution::can_make_pali_queries(s, queries);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
