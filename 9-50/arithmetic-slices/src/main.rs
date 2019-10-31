use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let mut cur_len: i32 = 0;
        let mut is_arithmetic: bool = false;
        for i in 2..a.len() {
            if a[i] - a[i - 1] == a[i - 1] - a[i - 2] {
                if is_arithmetic {
                    cur_len += 1;
                } else {
                    is_arithmetic = true;
                    cur_len = 3;
                }
            } else {
                if is_arithmetic {
                    is_arithmetic = false;
                    for j in 2..cur_len {
                        result += cur_len - j;
                    }
                    cur_len = 0;
                }
            }
        }
        if is_arithmetic {
            is_arithmetic = false;
            for j in 2..cur_len {
                result += cur_len - j;
            }
            cur_len = 0;
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
            a.push(number);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Number of arithmetic slices: {}", Solution::number_of_arithmetic_slices(a));
}
