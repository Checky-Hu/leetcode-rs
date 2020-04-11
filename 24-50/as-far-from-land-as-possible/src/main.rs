use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let len: usize = grid.len();
        let mut count: usize = 0;
        let mut flags: Vec<Vec<bool>> = vec![vec![false; len]; len];
        let mut queue: Vec<Vec<usize>> = Vec::new();
        for (i, row) in grid.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == 1 {
                    count += 1;
                    flags[i][j] = true;
                    queue.push(vec![i, j]);
                }
            }
        }
        if count == 0 || count == len * len {
            return -1;
        }
        let mut result: i32 = 0;
        while count < len * len {
            result += 1;
            let mut next: Vec<Vec<usize>> = Vec::new();
            for pos in queue.iter() {
                if pos[0] > 0 && !flags[pos[0] - 1][pos[1]] {
                    flags[pos[0] - 1][pos[1]] = true;
                    next.push(vec![pos[0] - 1, pos[1]]);
                    count += 1;
                }
                if pos[1] + 1 < len && !flags[pos[0]][pos[1] + 1] {
                    flags[pos[0]][pos[1] + 1] = true;
                    next.push(vec![pos[0], pos[1] + 1]);
                    count += 1;
                }
                if pos[0] + 1 < len && !flags[pos[0] + 1][pos[1]] {
                    flags[pos[0] + 1][pos[1]] = true;
                    next.push(vec![pos[0] + 1, pos[1]]);
                    count += 1;
                }
                if pos[1] > 0 && !flags[pos[0]][pos[1] - 1] {
                    flags[pos[0]][pos[1] - 1] = true;
                    next.push(vec![pos[0], pos[1] - 1]);
                    count += 1;
                }
            }
            queue = next;
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

    if 0 == ret || n * n != ret {
        println!("Require at least (1 + arg1 ^ 2) parameters.");
        return;
    }

    println!(
        "Maximum distance from land: {}",
        Solution::max_distance(grid)
    );
}
