use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
        let mut status: Vec<i32> = vec![0; n as usize];
        for pair in pairs.iter() {
            status[pair[0] as usize] = pair[1];
            status[pair[1] as usize] = pair[0];
        }
        let mut result: i32 = 0;
        for i in 0..n {
            let mut is_unhappy: bool = false;
            for current in preferences[i as usize].iter() {
                if *current == status[i as usize] {
                    break;
                } else {
                    for next in preferences[*current as usize].iter() {
                        if *next == i {
                            is_unhappy = true;
                            break;
                        } else if *next == status[*current as usize] {
                            break;
                        }
                    }
                    if is_unhappy {
                        break;
                    }
                }
            }
            if is_unhappy {
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: usize = 0;
    let mut preferences: Vec<Vec<i32>> = Vec::new();
    let mut pairs: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = usize::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(t);
                if preferences.len() == n {
                    if tmp_row.len() == 2 {
                        pairs.push(tmp_row);
                        tmp_row = Vec::new();
                    }
                } else if tmp_row.len() == n - 1 {
                    preferences.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if 0 == ret || n == 0 || n & 1 == 1 || preferences.len() != n || pairs.len() != n >> 1 {
        println!("Require at least (1 + arg1 * arg1) parameters.");
        return;
    }

    println!(
        "Unhappy friends: {}",
        Solution::unhappy_friends(n as i32, preferences, pairs)
    );
}
