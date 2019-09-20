use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut result: i32 = 0;
        let mut flags:Vec<bool> = vec![true; n as usize];
        let mut i: i32 = 2;
        while i < n {
            if flags[i as usize] {
                result += 1;
                let mut j: i32 = 2;
                while i * j < n {
                    flags[(i * j) as usize] = false;
                    j += 1;
                }
            }
            i += 1;
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Primes: {}", Solution::count_primes(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
