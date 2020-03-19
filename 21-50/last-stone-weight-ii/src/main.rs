use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let len: usize = stones.len();
        let mut sum: i32 = 0;
        for stone in stones.iter() {
            sum += *stone;
        }
        let mut dp: Vec<i32> = vec![0; sum as usize / 2 + 1];
        dp[0] = 1;
        for i in 0..len {
            let mut weight: usize = sum as usize / 2;
            loop {
                if (stones[i] as usize) <= weight {
                    dp[weight] += dp[weight - stones[i] as usize];
                }
                if weight == 0 {
                    break;
                } else {
                    weight -= 1;
                }
            }
        }
        let mut result: i32 = i32::max_value();
        for (i, v) in dp.iter().enumerate() {
            if *v > 0 {
                let t: i32 = sum - i as i32 * 2;
                if t < result {
                    result = t;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut stones: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                stones.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Last stone weight: {}",
        Solution::last_stone_weight_ii(stones)
    );
}
