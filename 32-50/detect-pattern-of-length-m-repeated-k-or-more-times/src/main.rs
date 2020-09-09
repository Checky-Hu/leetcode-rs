use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let len: usize = arr.len();
        let min: usize = (m * k) as usize;
        if len < min {
            return false;
        }
        for a in 0..=(len - min) {
            let mut found: bool = true;
            for b in 0..m {
                for c in 1..k {
                    if arr[a + b as usize] != arr[a + (b + c * m) as usize] {
                        found = false;
                        break;
                    }
                }
                if !found {
                    break;
                }
            }
            if found {
                return true;
            }
        }
        false
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    let mut m: i32 = 0;
    let mut k: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => m = i32::from_str(&arg).expect("Error parse."),
            2 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 3 parameters.");
        return;
    }

    println!("Detect pattern: {}", Solution::contains_pattern(arr, m, k));
}
