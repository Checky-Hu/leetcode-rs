use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let len: usize = pushed.len();
        let mut stack: Vec<i32> = Vec::with_capacity(len);
        let mut i1: usize = 0;
        let mut i2: usize = 0;
        loop {
            let mut push: bool = true;
            if let Some(x) = stack.last() {
                if i2 == len {
                    break;
                }
                if *x == popped[i2] {
                    push = false;
                    stack.pop();
                    i2 += 1;
                }
            }
            if push {
                if i1 == len {
                    break;
                }
                stack.push(pushed[i1]);
                i1 += 1;
            }
        }
        i1 == len && i2 == len
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut pushed: Vec<i32> = Vec::new();
    let mut popped: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                if pushed.len() == n as usize {
                    popped.push(t);
                } else {
                    pushed.push(t);
                }
            }
        }
    }

    if 0 == n || 0 == ret {
        println!("Require at least (1 + 2 * arg1) parameters.");
        return;
    }

    println!(
        "Valid stack sequences: {}",
        Solution::validate_stack_sequences(pushed, popped)
    );
}
