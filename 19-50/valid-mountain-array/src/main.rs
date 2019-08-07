use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn valid_mountain_array(a: Vec<i32>) -> bool {
        let len: usize = a.len();
        if len < 3 {
            return false
        }

        let mut reach_max: bool = false;
        let mut i: usize = 1;
        while i < len {
            if a[i - 1] == a[i] {
                return false
            } else if a[i - 1] < a[i] {
                if reach_max {
                    return false
                }
            } else {
                if i == 1 {
                    return false
                } else {
                    if !reach_max {
                        reach_max = true;
                    }
                }
            }
            i += 1;
        }
        reach_max
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            a.push(n);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Mountain: {}", Solution::valid_mountain_array(a));
}

