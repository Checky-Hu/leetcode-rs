use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let len: usize = grid.len();
        let mut max_tb: Vec<i32> = vec![0; len];
        let mut max_lr: Vec<i32> = vec![0; len];
        for (i, x) in grid.iter().enumerate() {
            for (j, y) in x.iter().enumerate() {
                if *y > max_tb[j] {
                    max_tb[j] = *y;
                }
                if *y > max_lr[i] {
                    max_lr[i] = *y;
                }
            }
        }
        let mut result: i32 = 0;
        for (i, x) in grid.iter().enumerate() {
            for (j, y) in x.iter().enumerate() {
                result += if max_tb[j] > max_lr[i] {
                    max_lr[i]
                } else {
                    max_tb[j]
                } - *y;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut grid: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(number);
                if tmp_row.len() == n as usize {
                    grid.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if 0 == n || n * n != ret {
        println!("Require at least (1 + arg1 * arg1) parameters.");
        return;
    }

    println!(
        "Max increase: {}",
        Solution::max_increase_keeping_skyline(grid)
    );
}
