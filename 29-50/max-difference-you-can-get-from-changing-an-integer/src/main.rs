use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn get_replaced_num(vec: &[i32], len: usize, replaced_n: i32, forbid_n: i32) -> i32 {
        let mut result: i32 = 0;
        let mut target: i32 = -1;
        let mut index: usize = len;
        loop {
            if target > -1 {
                if target == vec[index] {
                    result = result * 10 + replaced_n;
                } else {
                    result = result * 10 + vec[index];
                }
            } else if vec[index] == forbid_n {
                result = result * 10 + vec[index];
            } else {
                result = result * 10 + replaced_n;
                if vec[index] != replaced_n {
                    target = vec[index];
                }
            }
            if index == 0 {
                break;
            } else {
                index -= 1;
            }
        }
        result
    }

    pub fn max_diff(num: i32) -> i32 {
        let mut vec: Vec<i32> = Vec::new();
        let mut n: i32 = num;
        while n > 0 {
            vec.push(n % 10);
            n /= 10;
        }
        let len: usize = vec.len();
        let a: i32 = Solution::get_replaced_num(&vec, len - 1, 9, -1);
        let b: i32 = if vec[len - 1] == 1 {
            if len == 1 {
                1
            } else {
                let mut t: i32 = 1;
                for _i in 1..len {
                    t *= 10;
                }
                t + Solution::get_replaced_num(&vec, len - 2, 0, 1)
            }
        } else {
            Solution::get_replaced_num(&vec, len - 1, 1, -1)
        };
        a - b
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let num: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Maximum difference: {}", Solution::max_diff(num));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
