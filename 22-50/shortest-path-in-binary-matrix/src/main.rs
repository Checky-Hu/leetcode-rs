use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let len: usize = grid.len();
        if grid[0][0] == 1 || grid[len - 1][len - 1] == 1 {
            return -1;
        }
        if len == 1 {
            return 1;
        }
        let deltas: Vec<(i32, i32)> = vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let mut visits: Vec<Vec<bool>> = vec![vec![false; len]; len];
        let mut queue: Vec<(i32, i32)> = vec![(0, 0)];
        let mut result: i32 = 2;
        while !queue.is_empty() {
            let mut next: Vec<(i32, i32)> = Vec::new();
            for node in queue {
                for delta in deltas.iter() {
                    let x: i32 = node.0 + delta.0;
                    let y: i32 = node.1 + delta.1;
                    if x >= 0
                        && x < len as i32
                        && y >= 0
                        && y < len as i32
                        && !visits[x as usize][y as usize]
                        && 0 == grid[x as usize][y as usize]
                    {
                        if x as usize == len - 1 && y as usize == len - 1 {
                            return result;
                        } else {
                            visits[x as usize][y as usize] = true;
                            next.push((x, y));
                        }
                    }
                }
            }
            queue = next;
            result += 1;
        }
        -1
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

    if ret == 0 {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Shortest path in binary matrix: {}",
        Solution::shortest_path_binary_matrix(grid)
    );
}
