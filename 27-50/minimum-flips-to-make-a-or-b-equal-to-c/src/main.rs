use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut mut_a: i32 = a;
        let mut mut_b: i32 = b;
        let mut mut_c: i32 = c;
        let mut result: i32 = 0;
        while mut_a > 0 || mut_b > 0 || mut_c > 0 {
            let bit_a: i32 = mut_a & 1;
            let bit_b: i32 = mut_b & 1;
            let bit_c: i32 = mut_c & 1;
            if bit_a | bit_b != bit_c {
                if bit_c == 0 {
                    result += bit_a + bit_b;
                } else {
                    result += 1;
                }
            }
            mut_a >>= 1;
            mut_b >>= 1;
            mut_c >>= 1;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => a = i32::from_str(&arg).expect("Error parse."),
            2 => b = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let c: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Minimum flips: {}", Solution::min_flips(a, b, c));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 3 parameters.");
    }
}
