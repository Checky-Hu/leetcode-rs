use std::cmp;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let col: usize = grid[0].len();
        // <max, min>
        let mut results: Vec<(i64, i64)> = Vec::new();
        for (i, r) in grid.iter().enumerate() {
            let mut current: Vec<(i64, i64)> = Vec::with_capacity(col);
            if i == 0 {
                let mut prefix: i64 = 1;
                for v in r.iter() {
                    prefix *= *v as i64;
                    current.push((prefix, prefix));
                }
            } else {
                for j in 0..col {
                    if j == 0 {
                        let t: i64 = results[0].0 * r[0] as i64;
                        current.push((t, t));
                    } else {
                        let t1: i64 = current[j - 1].0 * r[j] as i64;
                        let t2: i64 = current[j - 1].1 * r[j] as i64;
                        let t3: i64 = results[j].0 * r[j] as i64;
                        let t4: i64 = results[j].1 * r[j] as i64;
                        current.push((
                            cmp::max(t1, cmp::max(t2, cmp::max(t3, t4))),
                            cmp::min(t1, cmp::min(t2, cmp::min(t3, t4))),
                        ));
                    }
                }
            }
            results = current;
        }
        if results[col - 1].0 >= 0 {
            (results[col - 1].0 % 1_000_000_007) as i32
        } else {
            -1
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut col: usize = 0;
    let mut grid: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => col = usize::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(t);
                if tmp_row.len() == col {
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

    println!("Maximum product: {}", Solution::max_product_path(grid));
}
