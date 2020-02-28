use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let len: usize = stones.len();
        let mut visits: Vec<bool> = vec![false; len];
        let mut result: i32 = 0;
        for i in 0..len {
            if visits[i] {
                continue;
            }
            visits[i] = true;
            let mut x_set: HashSet<i32> = HashSet::new();
            x_set.insert(stones[i][0]);
            let mut y_set: HashSet<i32> = HashSet::new();
            y_set.insert(stones[i][1]);
            loop {
                let mut add: i32 = 0;
                for j in (i + 1)..len {
                    if visits[j] {
                        continue;
                    }
                    if x_set.contains(&stones[j][0]) || y_set.contains(&stones[j][1]) {
                        x_set.insert(stones[j][0]);
                        y_set.insert(stones[j][1]);
                        add += 1;
                        visits[j] = true;
                    }
                }
                if add == 0 {
                    break;
                } else {
                    result += add;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut stones: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(number);
                if tmp_row.len() == 2 {
                    stones.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if ret == 0 {
        println!("Require at least two parameters.");
        return;
    }

    println!("Most removed stones: {}", Solution::remove_stones(stones));
}
