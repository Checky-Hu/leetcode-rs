use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn helper(a: i32, b: i32, mem: &mut Vec<Vec<f64>>) -> f64 {
        if a <= 0 {
            if b <= 0 {
                0.5f64
            } else {
                1f64
            }
        } else if b <= 0 {
            0f64
        } else if mem[a as usize][b as usize] > 0f64 {
            mem[a as usize][b as usize]
        } else {
            mem[a as usize][b as usize] = Solution::helper(a - 4, b, mem) * 0.25f64
                + Solution::helper(a - 3, b - 1, mem) * 0.25f64
                + Solution::helper(a - 2, b - 2, mem) * 0.25f64
                + Solution::helper(a - 1, b - 3, mem) * 0.25f64;
            mem[a as usize][b as usize]
        }
    }

    pub fn soup_servings(n: i32) -> f64 {
        if n >= 4800 {
            1f64
        } else {
            let mut mem: Vec<Vec<f64>> = vec![vec![0f64; 200]; 200];
            let parts: i32 = (n + 24) / 25;
            Solution::helper(parts, parts, &mut mem)
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Soup servings: {}", Solution::soup_servings(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
