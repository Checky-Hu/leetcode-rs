use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let row: usize = grid.len();
        let col: usize = grid[0].len();
        let mut visits: Vec<Vec<bool>> = vec![vec![false; col]; row];
        let mut result: i32 = 0;
        for i in 0..row {
            for j in 0..col {
                if !visits[i][j] && grid[i][j] == 0 {
                    visits[i][j] = true;
                    let mut is_closed: bool = true;
                    let mut queue: Vec<(usize, usize)> = vec![(i, j)];
                    while !queue.is_empty() {
                        let mut next: Vec<(usize, usize)> = Vec::new();
                        for pos in queue {
                            if pos.0 == 0 || pos.0 + 1 == row || pos.1 == 0 || pos.1 + 1 == col {
                                is_closed = false;
                            }
                            if pos.0 > 0 && !visits[pos.0 - 1][pos.1] && grid[pos.0 - 1][pos.1] == 0
                            {
                                visits[pos.0 - 1][pos.1] = true;
                                next.push((pos.0 - 1, pos.1));
                            }
                            if pos.0 + 1 < row
                                && !visits[pos.0 + 1][pos.1]
                                && grid[pos.0 + 1][pos.1] == 0
                            {
                                visits[pos.0 + 1][pos.1] = true;
                                next.push((pos.0 + 1, pos.1));
                            }
                            if pos.1 > 0 && !visits[pos.0][pos.1 - 1] && grid[pos.0][pos.1 - 1] == 0
                            {
                                visits[pos.0][pos.1 - 1] = true;
                                next.push((pos.0, pos.1 - 1));
                            }
                            if pos.1 + 1 < col
                                && !visits[pos.0][pos.1 + 1]
                                && grid[pos.0][pos.1 + 1] == 0
                            {
                                visits[pos.0][pos.1 + 1] = true;
                                next.push((pos.0, pos.1 + 1));
                            }
                        }
                        queue = next;
                    }
                    if is_closed {
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

    if row == 0 || col == 0 || row * col != ret {
        println!("Require at least (2 + arg1 * arg2) parameters.");
        return;
    }

    println!("Number of closed island: {}", Solution::closed_island(grid));
}
