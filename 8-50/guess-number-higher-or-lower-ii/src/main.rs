use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let len: usize = n as usize;
        let mut result: Vec<Vec<i32>> = vec![vec![0; len + 1]; len + 1];
        for i in 2..=len {
            let mut j: usize = i - 1;
            while j > 0 {
                let mut min: i32 = i32::max_value();
                for k in (j + 1)..i {
                    let tmp: i32 = k as i32 + if result[j][k - 1] >= result[k + 1][i] {
                        result[j][k - 1]
                    } else {
                        result[k + 1][i]
                    };
                    if min > tmp {
                        min = tmp;
                    }
                }
                result[j][i] = if j + 1 == i {
                    j as i32
                } else {
                    min
                };
                j -= 1;
            }
        }
        result[1][len]
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Amount: {}", Solution::get_money_amount(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
