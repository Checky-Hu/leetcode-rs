use std::env;
use std::mem;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let len: usize = grid.len();
        let mut indices: Vec<usize> = Vec::with_capacity(len);
        for row in grid.iter() {
            let mut index: usize = len - 1;
            let mut count: usize = 0;
            loop {
                if row[index] == 0 {
                    count += 1;
                    if index == 0 {
                        break;
                    } else {
                        index -= 1;
                    }
                } else {
                    break;
                }
            }
            indices.push(count);
        }
        let mut result: i32 = 0;
        for i in 0..len {
            let target: usize = len - 1 - i;
            if indices[i] < target {
                let mut index: usize = i + 1;
                let mut value: usize = indices[i];
                let mut is_valid: bool = false;
                while index < len {
                    mem::swap(&mut value, &mut indices[index]);
                    result += 1;
                    if value >= target {
                        is_valid = true;
                        break;
                    } else {
                        index += 1;
                    }
                }
                if !is_valid {
                    return -1;
                }
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
        println!("Require at least (1 + arg1 * arg1) parameters.");
        return;
    }

    println!(
        "Minimum swaps to arrange a binary grid: {}",
        Solution::min_swaps(grid)
    );
}
