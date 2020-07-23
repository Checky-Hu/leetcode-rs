use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut stacks: Vec<i32> = vec![0];
        let len: usize = prices.len();
        let mut result: Vec<i32> = vec![0; len];
        let mut i: usize = len - 1;
        loop {
            let mut price: i32 = prices[i];
            while let Some(x) = stacks.last() {
                if *x > prices[i] {
                    stacks.pop();
                } else {
                    price -= *x;
                    break;
                }
            }
            result[i] = price;
            stacks.push(prices[i]);
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut prices: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                prices.push(t);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    let result: Vec<i32> = Solution::final_prices(prices);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
