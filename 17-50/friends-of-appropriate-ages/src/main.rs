extern crate quicksort;

use quicksort::qsi32;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let mut tmp: Vec<i32> = ages;
        let len: usize = tmp.len();
        qsi32::quick_sort(&mut tmp, 0, len - 1);
        let mut result: i32 = 0;
        let mut i: usize = len - 1;
        loop {
            if i == 0 {
                break;
            }
            let mut j: usize = i - 1;
            loop {
                if tmp[j] <= tmp[i] / 2 + 7 {
                    break;
                } else if tmp[j] == tmp[i] {
                    result += 2;
                } else {
                    result += 1;
                }
                if j == 0 {
                    break;
                } else {
                    j -= 1;
                }
            }
            i -= 1;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut ages: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let age: i32 = i32::from_str(&arg).expect("Error parse.");
                ages.push(age);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Friend requests: {}", Solution::num_friend_requests(ages));
}
