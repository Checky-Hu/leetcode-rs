use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let sqrt: i32 = (n as f64).sqrt() as i32;
        let mut result: Vec<i32> = Vec::new();
        for i in 1..=sqrt {
            if n % i == 0 {
                result.push(i);
                let j: i32 = n / i;
                if i != j {
                    result.push(j);
                }
            }
        }
        let len: usize = result.len();
        if len < k as usize {
            return -1;
        }
        let mut forward: bool = true;
        let mut index: usize = 0;
        for _i in 1..k {
            if forward {
                index += 2;
            } else {
                index -= 2;
            }
            if index >= len {
                index -= if len & 1 == 0 { 1 } else { 3 };
                forward = false;
            }
        }
        result[index]
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
                println!("The kth factor: {}", Solution::kth_factor(n, k));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
