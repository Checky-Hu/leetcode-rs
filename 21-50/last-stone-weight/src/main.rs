extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut len: usize = stones.len();
        let mut tmp: Vec<i32> = stones.clone();
        qsi32::quick_sort(&mut tmp, 0, len - 1);
        while len > 1 {
            let max: i32 = tmp.pop().unwrap();
            let second_max: i32 = tmp.pop().unwrap();
            len -= 2;
            let new_stone: i32 = max - second_max;
            if new_stone != 0 {
                let mut i: usize = 0;
                while i < len {
                    if tmp[i] >= new_stone {
                        break;
                    } else {
                        i += 1;
                    }
                }
                tmp.insert(i, new_stone);
                len += 1;
            }
        }
        if len == 1 {
            tmp[0]
        } else {
            0
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut stones: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                stones.push(n);
            },
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Last stone weight: {}", Solution::last_stone_weight(stones));
}

