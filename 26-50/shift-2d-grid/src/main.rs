use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let r: usize = grid.len();
        let c: usize = grid[0].len();
        let total: usize = r * c;
        let shift: usize = k as usize % total;
        let mut result: Vec<Vec<i32>> = vec![vec![0; c]; r];
        for (i, row) in grid.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                let target: usize = (i * c + j + shift) % total;
                let new_i: usize = target / c;
                let new_j: usize = target - new_i * c;
                result[new_i][new_j] = *value;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut n: i32 = 0;
    let mut grid: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            2 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(t);
                if tmp.len() == n as usize {
                    grid.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    let result: Vec<Vec<i32>> = Solution::shift_grid(grid, k);
    for row in result {
        for t in row {
            print!("{} ", t);
        }
        println!();
    }
}
