use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let len: usize = arr.len();
        let mut max: i32 = -1;
        let mut max_pos: usize = 0;
        for i in 0..len {
            if i == max_pos {
                max = -1;
                for (j, v) in arr.iter().enumerate().take(len).skip(i + 1) {
                    if *v > max {
                        max = *v;
                        max_pos = j;
                    }
                }
            }
            result.push(max);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<i32> = Solution::replace_elements(arr);
    for r in &result {
        print!("{} ", *r);
    }
    println!();
}
