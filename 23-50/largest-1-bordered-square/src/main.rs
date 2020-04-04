use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn largest_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        let row: usize = grid.len();
        let col: usize = grid[0].len();
        let mut top: Vec<Vec<usize>> = vec![vec![0; col]; row];
        let mut left: Vec<Vec<usize>> = vec![vec![0; col]; row];
        for i in 0..row {
            for j in 0..col {
                if grid[i][j] == 1 {
                    top[i][j] = if i == 0 { 1 } else { top[i - 1][j] + 1 };
                    left[i][j] = if j == 0 { 1 } else { left[i][j - 1] + 1 };
                }
            }
        }
        let mut result: i32 = 0;
        let mut i: usize = row - 1;
        loop {
            let mut j: usize = col - 1;
            loop {
                let mut tmp: usize = if top[i][j] > left[i][j] {
                    left[i][j]
                } else {
                    top[i][j]
                };
                while tmp as i32 > result {
                    if top[i][j + 1 - tmp] >= tmp && left[i + 1 - tmp][j] >= tmp {
                        result = tmp as i32;
                        break;
                    } else {
                        tmp -= 1;
                    }
                }
                if j == 0 {
                    break;
                } else {
                    j -= 1;
                }
            }
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        result * result
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

    println!(
        "Largest bordered square: {}",
        Solution::largest_bordered_square(grid)
    );
}
