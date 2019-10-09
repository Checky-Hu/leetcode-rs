use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut result: Vec<i32> = vec![amount + 1; amount as usize + 1];
        result[0] = 0;
        for i in 1..=amount {
            for c in &coins {
                if i >= *c {
                    let t: i32 = result[(i - *c) as usize] + 1;
                    if t < result[i as usize] {
                        result[i as usize] = t;
                    }
                }
            }
        }
        if result[amount as usize] < amount + 1 {
            result[amount as usize]
        } else {
            -1
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut amount: i32 = 0;
    let mut coins: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => amount = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let coin: i32 = i32::from_str(&arg).expect("Error parse.");
                coins.push(coin);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return
    }

    println!("Least coins: {}", Solution::coin_change(coins, amount));
}
