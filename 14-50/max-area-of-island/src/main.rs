use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut result: i32 = 0;
        let r: usize = grid.len();
        let c: usize = grid[0].len();
        let mut flags: Vec<Vec<bool>> = vec![vec![false; c]; r];
        for i in 0..r {
            for j in 0..c {
                if !flags[i][j] {
                    flags[i][j] = true;
                    if grid[i][j] == 1 {
                        let mut count: i32 = 1;
                        let mut tmp: Vec<Vec<usize>> = vec![vec![i, j]];
                        while !tmp.is_empty() {
                            let mut next: Vec<Vec<usize>> = Vec::new();
                            for pos in tmp {
                                if pos[0] > 0
                                    && !flags[pos[0] - 1][pos[1]]
                                    && grid[pos[0] - 1][pos[1]] == 1
                                {
                                    flags[pos[0] - 1][pos[1]] = true;
                                    next.push(vec![pos[0] - 1, pos[1]]);
                                    count += 1;
                                }
                                if pos[1] + 1 < c
                                    && !flags[pos[0]][pos[1] + 1]
                                    && grid[pos[0]][pos[1] + 1] == 1
                                {
                                    flags[pos[0]][pos[1] + 1] = true;
                                    next.push(vec![pos[0], pos[1] + 1]);
                                    count += 1;
                                }
                                if pos[0] + 1 < r
                                    && !flags[pos[0] + 1][pos[1]]
                                    && grid[pos[0] + 1][pos[1]] == 1
                                {
                                    flags[pos[0] + 1][pos[1]] = true;
                                    next.push(vec![pos[0] + 1, pos[1]]);
                                    count += 1;
                                }
                                if pos[1] > 0
                                    && !flags[pos[0]][pos[1] - 1]
                                    && grid[pos[0]][pos[1] - 1] == 1
                                {
                                    flags[pos[0]][pos[1] - 1] = true;
                                    next.push(vec![pos[0], pos[1] - 1]);
                                    count += 1;
                                }
                            }
                            tmp = next;
                        }
                        if count > result {
                            result = count;
                        }
                    }
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut grid: Vec<Vec<i32>> = Vec::new();
    let mut r: i32 = 0;
    let mut c: i32 = 0;
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => r = i32::from_str(&arg).expect("Error parse."),
            2 => c = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(number);
                if tmp_row.len() == c as usize {
                    grid.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if 0 == ret || r * c != ret {
        println!("Require at least (2 + arg1 * arg2) parameters.");
        return;
    }

    println!("Max area of island: {}", Solution::max_area_of_island(grid));
}
