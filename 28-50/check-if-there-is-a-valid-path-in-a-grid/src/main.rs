use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let row: i32 = grid.len() as i32;
        let col: i32 = grid[0].len() as i32;
        if row == 1 && col == 1 {
            return true;
        }
        let mut visit: Vec<Vec<bool>> = vec![vec![false; col as usize]; row as usize];
        visit[0][0] = true;
        // (row, col, dir)
        let mut queue: Vec<(i32, i32, i32)> = vec![(0, 0, -1)];
        // 0 -> up, 1 -> right, 2 -> down, 3 -> left)
        let match_pattern: Vec<Vec<i32>> =
            vec![vec![2, 3, 4], vec![1, 3, 5], vec![2, 5, 6], vec![1, 4, 6]];
        while !queue.is_empty() {
            let mut nexts: Vec<(i32, i32, i32)> = Vec::new();
            for cur in queue {
                let dirs = match grid[cur.0 as usize][cur.1 as usize] {
                    1 => {
                        if cur.2 == -1 {
                            vec![1]
                        } else {
                            vec![cur.2]
                        }
                    }
                    2 => {
                        if cur.2 == -1 {
                            vec![2]
                        } else {
                            vec![cur.2]
                        }
                    }
                    3 => {
                        if cur.2 == -1 || cur.2 == 1 {
                            vec![2]
                        } else {
                            vec![3]
                        }
                    }
                    4 => {
                        if cur.2 == -1 {
                            vec![1, 2]
                        } else if cur.2 == 3 {
                            vec![2]
                        } else {
                            vec![1]
                        }
                    }
                    5 => {
                        if cur.2 == -1 {
                            Vec::new()
                        } else if cur.2 == 2 {
                            vec![3]
                        } else {
                            vec![0]
                        }
                    }
                    _ => {
                        if cur.2 == -1 || cur.2 == 2 {
                            vec![1]
                        } else {
                            vec![0]
                        }
                    }
                };
                for dir in dirs.iter() {
                    let next: (i32, i32, i32) = match *dir {
                        0 => (cur.0 - 1, cur.1, *dir),
                        1 => (cur.0, cur.1 + 1, *dir),
                        2 => (cur.0 + 1, cur.1, *dir),
                        _ => (cur.0, cur.1 - 1, *dir),
                    };
                    if next.0 < 0
                        || next.0 >= row
                        || next.1 < 0
                        || next.1 >= col
                        || visit[next.0 as usize][next.1 as usize]
                    {
                        continue;
                    }
                    let mut is_match: bool = false;
                    for v in match_pattern[next.2 as usize].iter() {
                        if *v == grid[next.0 as usize][next.1 as usize] {
                            is_match = true;
                            break;
                        }
                    }
                    if is_match {
                        if next.0 == row - 1 && next.1 == col - 1 {
                            return true;
                        } else {
                            visit[next.0 as usize][next.1 as usize] = true;
                            nexts.push(next);
                        }
                    }
                }
            }
            queue = nexts;
        }
        false
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut col: i32 = 0;
    let mut grid: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => col = i32::from_str(&arg).expect("Error parse."),
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

    if 0 == col || 0 == ret || ret % col != 0 {
        println!("Require at least (1 + arg1 * n) parameters.");
        return;
    }

    println!("There is a valid path: {}", Solution::has_valid_path(grid));
}
