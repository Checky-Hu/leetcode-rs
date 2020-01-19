use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut tmp: Vec<i32> = Vec::new();
        let mut num: i32 = n;
        while num > 0 {
            tmp.push(num % 10);
            num /= 10;
        }
        let len: usize = tmp.len();
        let mut pos: usize = len;
        for i in 1..len {
            if tmp[i] > tmp[i - 1] {
                pos = i;
                tmp[i] -= 1;
            }
        }
        if pos < len {
            for t in tmp.iter_mut().take(pos) {
                *t = 9;
            }
        }
        let mut result: i32 = 0;
        let mut multi: i32 = 1;
        for t in tmp {
            result += t * multi;
            multi *= 10;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!(
                "Monotone increasing digits: {}",
                Solution::monotone_increasing_digits(n)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
