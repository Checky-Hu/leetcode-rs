use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, p: i32) -> i32 {
        let len: usize = tokens.len();
        if len == 0 {
            return 0;
        }
        let mut temp: Vec<i32> = tokens;
        temp.sort();
        let mut result: i32 = 0;
        let mut score: i32 = 0;
        let mut power: i32 = p;
        let mut left: usize = 0;
        let mut right: usize = len - 1;
        while left <= right {
            if temp[left] <= power {
                power -= temp[left];
                left += 1;
                score += 1;
            } else if score == 0 {
                break;
            } else {
                score -= 1;
                power += temp[right];
                right -= 1;
            }
            if score > result {
                result = score;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut p: i32 = -1;
    let mut tokens: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => p = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tokens.push(n);
            }
        }
    }

    if -1 == p || 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!(
        "Bag of tokens score: {}",
        Solution::bag_of_tokens_score(tokens, p)
    );
}
