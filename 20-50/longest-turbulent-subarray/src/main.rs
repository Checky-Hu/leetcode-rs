use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_turbulence_size(a: Vec<i32>) -> i32 {
        let mut result: i32 = 1;
        // (number, count, state).
        // state: 0 => whatever, 1 => less, 2 => great.
        let mut pre: (i32, i32, u8) = (a[0], 1, 0);
        for v in a.iter().skip(1) {
            match pre.0.cmp(v) {
                Ordering::Less => {
                    pre.0 = *v;
                    if pre.2 != 1 {
                        pre.1 += 1;
                    } else {
                        pre.1 = 2;
                    }
                    pre.2 = 1;
                }
                Ordering::Equal => {
                    pre.1 = 1;
                    pre.2 = 0;
                }
                Ordering::Greater => {
                    pre.0 = *v;
                    if pre.2 != 2 {
                        pre.1 += 1;
                    } else {
                        pre.1 = 2;
                    }
                    pre.2 = 2;
                }
            }
            if pre.1 > result {
                result = pre.1;
            }
        }
        result
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

    if ret == 0 {
        println!("Require at least one parameter.");
        return;
    }

    println!("Max turbulence size: {}", Solution::max_turbulence_size(a));
}
