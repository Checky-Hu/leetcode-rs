use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn matrix_score(a: Vec<Vec<i32>>) -> i32 {
        let r: usize = a.len();
        let c: usize = a[0].len();
        let mut result: i32 = r as i32 * (1 << (c - 1));
        for i in 1..c {
            let mut count_1: usize = 0;
            for v in a.iter().take(r) {
                if v[i] == v[0] {
                    count_1 += 1;
                }
            }
            result += (1 << (c - 1 - i))
                * if 2 * count_1 < r {
                    r - count_1
                } else {
                    count_1
                } as i32;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut rows: i32 = 0;
    let mut columns: i32 = 0;
    let mut a: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => rows = i32::from_str(&arg).expect("Error parse."),
            2 => columns = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(number);
                if ret % columns == 0 {
                    a.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if 0 == ret || rows * columns != ret {
        println!("Require at least (2 + arg1 * arg2) parameters.");
        return;
    }

    println!(
        "Max score after flipping matrix: {}",
        Solution::matrix_score(a)
    );
}
