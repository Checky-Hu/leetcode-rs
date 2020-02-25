use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_falling_path_sum(a: Vec<Vec<i32>>) -> i32 {
        let len: usize = a.len();
        let mut min: Vec<i32> = a[0].clone();
        for i in 1..len {
            let mut tmp: Vec<i32> = Vec::with_capacity(len);
            for j in 0..len {
                let mut t: i32 = min[j];
                if j > 0 && min[j - 1] < t {
                    t = min[j - 1];
                }
                if j < len - 1 && min[j + 1] < t {
                    t = min[j + 1];
                }
                tmp.push(a[i][j] + t);
            }
            min = tmp;
        }
        let mut result: i32 = i32::max_value();
        for v in min.iter() {
            if *v < result {
                result = *v;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut a: Vec<Vec<i32>> = Vec::new();
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
                    a.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if 0 == n || n * n > ret {
        println!("Require at least (1 + arg1 ^ 2) parameters.");
        return;
    }

    println!(
        "Min falling path sum: {}",
        Solution::min_falling_path_sum(a)
    );
}
