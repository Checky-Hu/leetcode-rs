use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        if n == 1 {
            return vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        }
        let mut result: Vec<i32> = Vec::new();
        for i in 1..=9 {
            let mut prefix: Vec<i32> = vec![i];
            for _j in 0..(n - 1) {
                let mut next: Vec<i32> = Vec::new();
                for p in prefix {
                    let mut t: i32 = p % 10;
                    t += k;
                    if t <= 9 {
                        next.push(p * 10 + t);
                    }
                    if k == 0 {
                        continue;
                    }
                    t -= 2 * k;
                    if t >= 0 {
                        next.push(p * 10 + t);
                    }
                }
                prefix = next;
            }
            result.append(&mut prefix);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let k: i32 = i32::from_str(&arg).expect("Error parse.");
                let result: Vec<i32> = Solution::nums_same_consec_diff(n, k);
                for r in result.iter() {
                    print!("{} ", *r);
                }
                println!();
                break;
            }
        }
    }

    if ret == 0 {
        println!("Require at least two parameters.");
    }
}
