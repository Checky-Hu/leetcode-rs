use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(2);
        let mut t: i32 = tomato_slices - 2 * cheese_slices;
        if t >= 0 && t & 1 == 0 {
            t >>= 1;
            if t <= cheese_slices {
                result.push(t);
                result.push(cheese_slices - t);
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut tomato_slices: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => tomato_slices = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let cheese_slices: i32 = i32::from_str(&arg).expect("Error parse.");
                let result: Vec<i32> = Solution::num_of_burgers(tomato_slices, cheese_slices);
                for r in result.iter() {
                    println!("{}", *r);
                }
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
