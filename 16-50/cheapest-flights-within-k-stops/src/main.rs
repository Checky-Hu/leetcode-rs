use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![-1; n as usize]; k as usize + 2];
        dp[0][src as usize] = 0;
        for i in 1..(k + 2) {
            dp[i as usize][src as usize] = 0;
            for flight in &flights {
                let tmp: i32 = if dp[i as usize - 1][flight[0] as usize] < 0 {
                    -1
                } else {
                    dp[i as usize - 1][flight[0] as usize] + flight[2]
                };
                if tmp > 0
                    && (dp[i as usize][flight[1] as usize] < 0
                        || dp[i as usize][flight[1] as usize] > tmp)
                {
                    dp[i as usize][flight[1] as usize] = tmp;
                }
            }
        }
        dp[k as usize + 1][dst as usize]
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut src: i32 = 0;
    let mut dst: i32 = 0;
    let mut k: i32 = 0;
    let mut flights: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            2 => src = i32::from_str(&arg).expect("Error parse."),
            3 => dst = i32::from_str(&arg).expect("Error parse."),
            4 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let v: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(v);
                if tmp.len() == 3 {
                    flights.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least four parameters.");
        return;
    }

    println!(
        "Cheapest price: {}",
        Solution::find_cheapest_price(n, flights, src, dst, k)
    );
}
