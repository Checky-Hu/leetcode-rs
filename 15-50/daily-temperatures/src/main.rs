use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let len: usize = t.len();
        let mut result: Vec<i32> = vec![0; len];
        let mut stack: Vec<usize> = Vec::new();
        for i in 0..len {
            while let Some(x) = stack.last() {
                if t[*x] < t[i] {
                    result[*x] = (i - *x) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut t: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                t.push(number);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<i32> = Solution::daily_temperatures(t);
    for r in &result {
        print!("{} ", r);
    }
    println!();
}
