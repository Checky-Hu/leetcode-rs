use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let len: usize = arr.len();
        if len < 2 {
            return 0;
        }
        let mut result: i32 = 0;
        let mut stack: Vec<i32> = Vec::new();
        stack.push(i32::max_value());
        for a in arr.iter() {
            while let Some(x) = stack.last() {
                if *x <= *a {
                    let tmp: i32 = *x;
                    stack.pop();
                    let pre: i32 = *stack.last().unwrap();
                    result += tmp * if pre < *a { pre } else { *a };
                } else {
                    break;
                }
            }
            stack.push(*a);
        }
        while stack.len() > 2 {
            let tmp: i32 = stack.pop().unwrap();
            result += tmp * *stack.last().unwrap();
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

    println!(
        "Minimum cost tree from leaf values: {}",
        Solution::mct_from_leaf_values(arr)
    );
}
