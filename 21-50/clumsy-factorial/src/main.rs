use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        let mut result: i32 = 0;
        // 0 => '*', 1 => '/', 2 => '+', 3 => '-'.
        let mut next: u8 = 0;
        let mut n_mut = n;
        let mut cur_multi1: i32 = 0;
        let mut cur_multi2: i32 = 0;
        let mut is_subtract: bool = false;
        while n_mut > 0 {
            match next {
                0 => cur_multi1 = n_mut,
                1 => cur_multi2 = n_mut,
                2 => {
                    let t: i32 = cur_multi1 * cur_multi2 / n_mut;
                    if is_subtract {
                        result -= t;
                    } else {
                        result += t;
                    }
                    cur_multi1 = 0;
                    cur_multi2 = 0;
                }
                _ => {
                    is_subtract = true;
                    result += n_mut;
                }
            }
            n_mut -= 1;
            next = (next + 1) % 4;
        }
        match next {
            1 => {
                if is_subtract {
                    result - cur_multi1
                } else {
                    result + cur_multi1
                }
            }
            2 => {
                if is_subtract {
                    result - cur_multi1 * cur_multi2
                } else {
                    result + cur_multi1 * cur_multi2
                }
            }
            _ => result,
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Clumsy of {}: {}", n, Solution::clumsy(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
