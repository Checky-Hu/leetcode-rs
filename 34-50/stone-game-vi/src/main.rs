use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let len: usize = alice_values.len();
        let mut sum: Vec<(i32, usize)> = Vec::with_capacity(len);
        for i in 0..len {
            sum.push((alice_values[i] + bob_values[i], i));
        }
        sum.sort_by(|a, b| b.0.cmp(&a.0));
        let mut a_score: i32 = 0;
        let mut b_score: i32 = 0;
        for i in 0..len {
            if i & 1 == 0 {
                a_score += alice_values[sum[i].1];
            } else {
                b_score += bob_values[sum[i].1];
            }
        }
        match a_score.cmp(&b_score) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut len: usize = 0;
    let mut alice_values: Vec<i32> = Vec::new();
    let mut bob_values: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => len = usize::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                if alice_values.len() == len {
                    bob_values.push(n);
                } else {
                    alice_values.push(n);
                }
            }
        }
    }

    if 0 == ret || len == 0 || ret != len * 2 {
        println!("Require at least (1 + 2 * arg1) parameters.");
        return;
    }

    println!(
        "Game result: {}",
        Solution::stone_game_vi(alice_values, bob_values)
    );
}
