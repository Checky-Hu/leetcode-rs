use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let mut result: Vec<char> = vec!['0'];
        let mut counts: usize = 1;
        while counts < k as usize {
            result.push('1');
            let mut i: usize = counts - 1;
            loop {
                result.push(if result[i] == '1' { '0' } else { '1' });
                if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
            }
            counts = counts * 2 + 1;
        }
        result[k as usize - 1]
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let k: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Kth bit: {}", Solution::find_kth_bit(n, k));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
