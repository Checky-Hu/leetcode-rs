use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        let mut n_mut: i32 = n;
        loop {
            let mut tmp_s: String = String::new();
            let mut tmp_n: i32 = n_mut;
            while tmp_n > 0 {
                tmp_s.insert(0, if tmp_n & 1 == 1 { '1' } else { '0' });
                tmp_n >>= 1;
            }
            if !s.contains(&tmp_s) {
                return false;
            }
            if n_mut == 1 {
                break;
            } else {
                n_mut -= 1;
            }
        }
        true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s: String = String::new();
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            s = arg;
        } else if 2 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!(
                "String contains substrings representing 1 to {}: {}",
                n,
                Solution::query_string(s, n)
            );
            break;
        }
    }

    if 2 > ret {
        println!("Require at least two parameters.");
    }
}
