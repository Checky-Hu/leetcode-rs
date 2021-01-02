use std::env;

struct Solution {}

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut result: char = '0';
        for c in n.chars() {
            if c > result {
                result = c;
            }
        }
        result as i32 - 48
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: String = arg;
                println!("Minimum partitions: {}", Solution::min_partitions(n));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
