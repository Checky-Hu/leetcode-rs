use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let len: usize = rating.len();
        if len < 3 {
            return 0;
        }
        let mut result: i32 = 0;
        for i in 0..(len - 2) {
            for j in (i + 1)..(len - 1) {
                for k in (j + 1)..len {
                    if (rating[i] < rating[j] && rating[j] < rating[k])
                        || (rating[i] > rating[j] && rating[j] > rating[k])
                    {
                        result += 1;
                    }
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut rating: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let rate: i32 = i32::from_str(&arg).expect("Error parse.");
                rating.push(rate);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }

    println!("Number of teams: {}", Solution::num_teams(rating));
}
