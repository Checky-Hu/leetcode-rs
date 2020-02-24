use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_subarrays_with_sum(a: Vec<i32>, s: i32) -> i32 {
        let len: usize = a.len();
        let mut result: i32 = 0;
        for i in 0..len {
            let mut sum: i32 = 0;
            let mut j: usize = i;
            while j < len {
                sum += a[j];
                if sum >= s {
                    break;
                } else {
                    j += 1;
                }
            }
            if j == len {
                break;
            } else if sum > s {
                continue;
            } else {
                let mut k: usize = j + 1;
                while k < len {
                    if a[k] == 1 {
                        break;
                    } else {
                        k += 1;
                    }
                }
                println!("r:{}", result);
                result += (k - j) as i32;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s: i32 = -1;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                a.push(n);
            }
        }
    }

    if -1 == s || 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!(
        "Number of subarrays: {}",
        Solution::num_subarrays_with_sum(a, s)
    );
}
