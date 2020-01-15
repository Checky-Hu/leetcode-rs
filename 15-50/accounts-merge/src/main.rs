use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        accounts
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut accounts: Vec<Vec<String>> = Vec::new();
    let mut tmp: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                if n == 0 {
                    n = i32::from_str(&arg).expect("Error parse.");
                } else {
                    let s: String = arg;
                    tmp.push(s);
                    if tmp.len() == n as usize {
                        accounts.push(tmp);
                        tmp = Vec::new();
                        n = 0;
                    }
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }

    let result: Vec<Vec<String>> = Solution::accounts_merge(accounts);
    for r in &result {
        for s in r {
            print!("{} ", s);
        }
        println!();
    }
}
