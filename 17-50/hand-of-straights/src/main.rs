extern crate quicksort;

use quicksort::qsi32;
use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, w: i32) -> bool {
        if hand.len() as i32 % w != 0 {
            return false;
        }
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut tmp: Vec<i32> = Vec::new();
        for v in &hand {
            match map.get_mut(v) {
                Some(x) => *x += 1,
                None => {
                    tmp.push(*v);
                    map.insert(*v, 1);
                }
            }
        }
        let len: usize = tmp.len();
        qsi32::quick_sort(&mut tmp, 0, len - 1);
        for t in &tmp {
            let mut count: i32 = 0;
            for v in 0..w {
                match map.get_mut(&(*t + v)) {
                    Some(x) => {
                        if v == 0 {
                            if *x == 0 {
                                break;
                            } else {
                                count = *x;
				*x = 0;
                            }
                        } else {
                            if *x < count {
                                return false;
                            } else {
                                *x -= count;
                            }
                        }
                    }
                    None => return false,
                }
            }
        }
        true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut w: i32 = 0;
    let mut hand: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => w = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                hand.push(t);
            }
        }
    }

    if 0 == w || 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!(
        "Is {} straight hand: {}",
        w,
        Solution::is_n_straight_hand(hand, w)
    );
}
