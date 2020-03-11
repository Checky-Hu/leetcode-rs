use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn base_neg2(n: i32) -> String {
        let mut n_mut: i32 = n;
        let mut result: String = String::new();
        while n_mut != 0 {
            result.insert(0, if n_mut & 1 == 1 { '1' } else { '0' });
            n_mut = 0 - (n_mut >> 1);
        }
        if result.is_empty() {
            "0".to_string()
        } else {
            result
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!(
                "Base negative 2 string of {}: {}",
                n,
                Solution::base_neg2(n)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
