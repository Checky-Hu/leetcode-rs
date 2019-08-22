use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
        let mut tmp: i32 = -1;
        let mut result: Vec<bool> = Vec::new();
        for n in a {
            if tmp < 0 {
                tmp = n;
            } else {
                tmp += n;
            }
            if tmp % 5 == 0 {
                result.push(true);
            } else {
                result.push(false);
            }
            tmp = (tmp % 10) << 1;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                a.push(n);
            },
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<bool> = Solution::prefixes_div_by5(a);
    for b in result {
        print!("{} ", b);
    }
    print!("\n");
}

