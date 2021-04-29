use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut costs_mut: Vec<i32> = costs;
        costs_mut.sort();
        let mut ret: i32 = 0;
        let mut sum: i32 = 0;
        for cost in costs_mut.iter() {
            sum += *cost;
            if sum > coins {
                break;
            } else {
                ret += 1;
            }
        }
        ret
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut coins: i32 = 0;
    let mut costs: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => coins = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let cost: i32 = i32::from_str(&arg).expect("Error parse.");
                costs.push(cost);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!("Max ice cream: {}", Solution::max_ice_cream(costs, coins));
}
