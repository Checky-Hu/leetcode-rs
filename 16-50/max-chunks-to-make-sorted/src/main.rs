use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let mut max: i32 = 0;
        for (i, v) in arr.iter().enumerate() {
            if max < *v {
                max = *v;
            }
            if max == i as i32 {
                result += 1;
            }
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

    println!("Max chunks: {}", Solution::max_chunks_to_sorted(arr));
}
