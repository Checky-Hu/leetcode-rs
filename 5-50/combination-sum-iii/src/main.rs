use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn combination_by_dfs(k: i32, n: i32, start: i32, mut out: &mut Vec<i32>, mut result: &mut Vec<Vec<i32>>) {
        if n < 0 {
            return
        }
        if n == 0 && out.len() == k as usize {
            result.push(out.clone());
            return
        }
        for i in start..=9 {
            out.push(i);
            Solution::combination_by_dfs(k, n - i, i + 1, &mut out, &mut result);
            out.pop();
        }
    }

    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut out: Vec<i32> = Vec::new();
        Solution::combination_by_dfs(k, n, 1, &mut out, &mut result);
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut n: i32 = 0;
    let mut k: i32;
    for (index, arg) in env::args().enumerate() {
        match index {
            1 => n = i32::from_str(&arg).expect("Error parse."),
            2 => {
                ret += 1;
                k = i32::from_str(&arg).expect("Error parse.");
                let result: Vec<Vec<i32>> = Solution::combination_sum3(n, k);
                for v in result {
                    for n in v {
                        print!("{} ", n);
                    }
                    print!("\n");
                }
            },
            _ => (),
        }
    }

    if 0 == ret {
        println!("Require at least two parameter.");
        return;
    }
}
