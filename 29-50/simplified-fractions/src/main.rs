use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for i in 1..n {
            for j in (i + 1)..=n {
                let mut a: i32 = j;
                let mut b: i32 = i;
                while b != 0 {
                    let t: i32 = a % b;
                    a = b;
                    b = t;
                }
                if a == 1 {
                    result.push(i.to_string() + "/" + &j.to_string());
                }
            }
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
            let result: Vec<String> = Solution::simplified_fractions(n);
            for r in result.iter() {
                print!("{} ", *r);
            }
            println!();
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
