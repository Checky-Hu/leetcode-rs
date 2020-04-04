use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let col: usize = grid[0].len();
        let mut index: usize = col;
        let mut result: i32 = 0;
        for row in grid.iter() {
            if index == col {
                for (i, v) in row.iter().enumerate() {
                    if *v < 0 {
                        index = i;
                        break;
                    }
                }
                result += (col - index) as i32;
            } else if index == 0 {
                result += col as i32;
            } else {
                let mut is_positive: bool = false;
                let mut j: usize = index - 1;
                loop {
                    if row[j] >= 0 {
                        is_positive = true;
                        break;
                    }
                    if j == 0 {
                        break;
                    } else {
                        j -= 1;
                    }
                }
                if is_positive {
                    index = j + 1;
                } else {
                    index = 0;
                }
                result += (col - index) as i32;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut col: i32 = 0;
    let mut grid: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => col = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(n);
                if tmp_row.len() == col as usize {
                    grid.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if 0 == ret || !tmp_row.is_empty() {
        println!("Require at least (1 + arg1 * n) parameters.");
        return;
    }

    println!("Negatives count: {}", Solution::count_negatives(grid));
}
