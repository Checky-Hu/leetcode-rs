use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut result: Vec<i32> = vec![0; amount as usize + 1];
        result[0] = 1;
        for c in &coins {
            for i in *c..=amount {
                result[i as usize] += result[(i - *c) as usize];
            }
        }
        result[amount as usize]
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

    println!("Ways to change: {}", Solution::change(amount, coins));
}
