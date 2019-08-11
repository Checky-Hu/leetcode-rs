extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn largest_perimeter(a: Vec<i32>) -> i32 {
        let len: usize = a.len();
        let mut tmp: Vec<i32> = a.clone();
        qsi32::quick_sort(&mut tmp, 0, len - 1);
        let mut result: i32 = 0;
        let mut i: usize = len - 3;
        loop {
            if tmp[i] + tmp[i + 1] > tmp[i + 2] {
                result = tmp[i] + tmp[i + 1] + tmp[i + 2];
                break;
            } else {
                if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
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

    if 3 > ret {
        println!("Require at least three parameters.");
        return;
    }

    println!("Largest perimeter: {}", Solution::largest_perimeter(a));
}

