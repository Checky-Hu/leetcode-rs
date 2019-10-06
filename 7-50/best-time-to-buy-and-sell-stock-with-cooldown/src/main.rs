use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy: i32 = i32::min_value();
        let mut pre_buy: i32;
        let mut sell: i32 = 0;
        let mut pre_sell: i32 = 0;
        for p in prices {
            pre_buy = buy;
            buy = if pre_sell - p >= pre_buy {
                pre_sell - p
            } else {
                pre_buy
            };
            pre_sell = sell;
            sell = if pre_buy + p >= pre_sell {
                pre_buy + p
            } else {
                pre_sell
            };
        }
        sell
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut prices: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                prices.push(number);
            },
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Max profit: {}", Solution::max_profit(prices));
}
