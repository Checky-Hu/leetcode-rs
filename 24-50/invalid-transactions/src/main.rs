use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let len: usize = transactions.len();
        let mut names: Vec<String> = Vec::with_capacity(len);
        let mut times: Vec<i32> = Vec::with_capacity(len);
        let mut amounts: Vec<i32> = Vec::with_capacity(len);
        let mut cities: Vec<String> = Vec::with_capacity(len);
        for transaction in transactions.iter() {
            let strs: Vec<&str> = transaction.split(',').collect();
            names.push(strs[0].to_string());
            times.push(i32::from_str(strs[1]).unwrap_or(0));
            amounts.push(i32::from_str(strs[2]).unwrap_or(0));
            cities.push(strs[3].to_string());
        }
        let mut result: Vec<String> = Vec::new();
        let mut visits: Vec<bool> = vec![false; len];
        for i in 0..len {
            let mut is_valid: bool = true;
            for j in (i + 1)..len {
                if names[i] == names[j]
                    && cities[i] != cities[j]
                    && (times[i] - times[j]).abs() <= 60
                {
                    is_valid = false;
                    if !visits[j] {
                        result.push(transactions[j].clone());
                        visits[j] = true;
                    }
                }
            }
            if !visits[i] && (!is_valid || amounts[i] > 1000) {
                result.push(transactions[i].clone());
                visits[i] = true;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut transactions: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let transaction: String = arg;
                transactions.push(transaction);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<String> = Solution::invalid_transactions(transactions);
    for r in result.iter() {
        println!("{}", *r);
    }
}
