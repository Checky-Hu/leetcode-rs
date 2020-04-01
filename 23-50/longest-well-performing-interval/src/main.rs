use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let len: usize = hours.len();
        let mut pre: i32 = 0;
        let mut sum: Vec<i32> = Vec::with_capacity(len + 1);
        sum.push(pre);
        for v in hours.iter() {
            pre += if *v > 8 { 1 } else { -1 };
            sum.push(pre);
        }
        let mut result: usize = 0;
        let mut i: usize = len;
        while i > result as usize {
            for j in 0..(i - result) {
                if sum[j] < sum[i] {
                    let t: usize = i - j;
                    if t > result {
                        result = t;
                    }
                    break;
                }
            }
            i -= 1;
        }
        result as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut hours: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                hours.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Longest well-performing intervals: {}",
        Solution::longest_wpi(hours)
    );
}
