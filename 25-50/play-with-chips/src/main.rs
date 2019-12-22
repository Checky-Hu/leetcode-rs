use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn min_cost_of_move_chips(chips: Vec<i32>) -> i32 {
        let mut odd_count: i32 = 0;
        let mut even_count: i32 = 0;
        for chip in chips {
            if chip & 1 == 0 {
                even_count += 1;
            } else {
                odd_count += 1;
            }
        }
        if even_count > odd_count {
            odd_count
        } else {
            even_count
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut chips: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let chip: i32 = i32::from_str(&arg).expect("Error parse.");
                chips.push(chip);
            },
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return
    }

    println!("Min cost of move chips: {}", Solution::min_cost_of_move_chips(chips));
}

