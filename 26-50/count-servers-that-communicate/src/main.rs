use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let row: usize = grid.len();
        let col: usize = grid[0].len();
        let mut result: i32 = 0;
        for i in 0..row {
            let mut count: i32 = 0;
            let mut pos: usize = col;
            for j in 0..col {
                if grid[i][j] == 1 {
                    count += 1;
                    pos = j;
                }
            }
            if count > 1 {
                result += count;
            } else if count == 1 {
                count = 0;
                for k in 0..row {
                    if grid[k][pos] == 1 {
                        count += 1;
                    }
                }
                if count > 1 {
                    result += 1;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut grid: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => row = i32::from_str(&arg).expect("Error parse."),
            2 => col = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(t);
                if tmp.len() == col as usize {
                    grid.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == row || 0 == col || row * col != ret {
        println!("Require at least (2 + arg1 * arg2) parameters.");
        return;
    }

    println!(
        "Servers that communicate: {}",
        Solution::count_servers(grid)
    );
}
