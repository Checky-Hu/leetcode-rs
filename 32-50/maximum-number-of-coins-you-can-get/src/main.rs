use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_coins(piles: Vec<i32>) -> i32 {
        let mut mut_piles: Vec<i32> = piles;
        mut_piles.sort();
        let len: usize = mut_piles.len();
        let mut index: usize = len / 3;
        let mut result: i32 = 0;
        while index < len {
            result += mut_piles[index];
            index += 2;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut piles: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let pile: i32 = i32::from_str(&arg).expect("Error parse.");
                piles.push(pile);
            }
        }
    }

    if 0 == ret || ret % 3 != 0 {
        println!("Require at least (3 * n) parameters.");
        return;
    }

    println!("Maximum number of coins: {}", Solution::max_coins(piles));
}
