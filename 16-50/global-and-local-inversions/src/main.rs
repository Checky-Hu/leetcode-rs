use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn is_ideal_permutation(a: Vec<i32>) -> bool {
        for i in 0..(a.len() - 1) {
            if a[i] > a[i + 1] && (a[i] != i as i32 + 1 || a[i + 1] != i as i32) {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                a.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Ideal permutation: {}", Solution::is_ideal_permutation(a));
}
