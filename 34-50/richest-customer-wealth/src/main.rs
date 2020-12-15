use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut result: i32 = 0;
        for account in accounts.iter() {
            let mut tmp: i32 = 0;
            for v in account.iter() {
                tmp += *v;
            }
            if tmp > result {
                result = tmp;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut len: usize = 0;
    let mut tmp: Vec<i32> = Vec::new();
    let mut accounts: Vec<Vec<i32>> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => len = usize::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(n);
                if tmp.len() == len {
                    accounts.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least (1 + arg1) parameters.");
        return;
    }

    println!("Maximum wealth: {}", Solution::maximum_wealth(accounts));
}
