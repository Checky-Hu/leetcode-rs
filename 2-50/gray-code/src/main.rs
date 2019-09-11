use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut len: i32 = 1;
        let mut i: i32 = 0;
        while i < n {
            len *= 2;
            i += 1;
        }
        let mut result: Vec<i32> = vec![0; len as usize];
        i = 0;
        while i < len {
            result[i as usize] = i ^ (i >> 1);
            i += 1;
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            let result: Vec<i32> = Solution::gray_code(n);
            for t in result {
                print!("{} ", t);
            }
            print!("\n");
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
