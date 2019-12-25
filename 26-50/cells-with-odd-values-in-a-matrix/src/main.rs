use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut rows: Vec<i32> = vec![0; n as usize];
        let mut cols: Vec<i32> = vec![0; m as usize];
        for indice in indices {
            rows[indice[0] as usize] += 1;
            cols[indice[1] as usize] += 1;
        }
        let mut result: i32 = 0;
        for i in 0..n {
            for j in 0..m {
                if (rows[i as usize] + cols[j as usize]) & 1 == 1 {
                    result += 1;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut m: i32 = 0;
    let mut indices: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            2 => m = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(t);
                if tmp.len() == 2 {
                    indices.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!("Odd cells count: {}", Solution::odd_cells(n, m, indices));
}
