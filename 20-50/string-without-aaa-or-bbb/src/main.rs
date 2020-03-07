use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn str_without3a3b(a: i32, b: i32) -> String {
        let (n, c1, c2, mut more, mut less) = if a >= b {
            (a - b, 'a', 'b', a, b)
        } else {
            (b - a, 'b', 'a', b, a)
        };
        let mut result: String = String::new();
        for _i in 0..n {
            result.push(c1);
            result.push(c1);
            result.push(c2);
            more -= 2;
            less -= 1;
            if more == 0 || less == 0 {
                break;
            }
        }
        while more > 0 || less > 0 {
            if more > 0 {
                result.push(c1);
                more -= 1;
            }
            if less > 0 {
                result.push(c2);
                less -= 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            a = i32::from_str(&arg).expect("Error parse.");
        } else if 2 == index {
            ret += 1;
            let b: i32 = i32::from_str(&arg).expect("Error parse.");
            println!(
                "String without 3a or 3b: {}",
                Solution::str_without3a3b(a, b)
            );
            break;
        }
    }

    if 2 > ret {
        println!("Require at least two parameters.");
    }
}
