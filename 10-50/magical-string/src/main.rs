use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        if n <= 0 {
            0
        } else if n <= 3 {
            1
        } else {
            let mut result: i32 = 1;
            let mut head: usize = 2;
            let mut tail: usize = 3;
            let mut num: i32 = 1;
            let mut v: Vec<i32> = vec![1, 2, 2];
            while tail < n as usize {
                for _i in 0..v[head] {
                    v.push(num);
                    if num == 1 && tail < n as usize {
                        result += 1;
                    }
                    tail += 1;
                }
                num ^= 3;
                head += 1;
            }
            result
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Number of 1 in magical string: {}", Solution::magical_string(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }
}
