use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let mut result: i64 = 0;
        while (result + 1) * result <= n as i64 * 2 {
            result += 1;
        }
        result as i32 - 1
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Rows: {}", Solution::arrange_coins(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
