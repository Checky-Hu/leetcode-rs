use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let row: usize = matrix.len();
        let col: usize = matrix[0].len();
        let mut full: i32 = 0;
        for _i in 0..col {
            full <<= 1;
            full += 1;
        }
        // Count the flips that make every row equals.
        let mut flips: HashMap<i32, i32> = HashMap::with_capacity(row);
        for vec in matrix.iter() {
            let mut t: i32 = 0;
            for v in vec.iter() {
                t <<= 1;
                t += *v;
            }
            match flips.get_mut(&t) {
                Some(x) => *x += 1,
                None => {
                    flips.insert(t, 1);
                }
            }
        }
        let mut result: i32 = 1;
        for (k, v) in flips.iter() {
            let t: i32 = *v
                + match flips.get(&(full ^ *k)) {
                    Some(x) => *x,
                    None => 0,
                };
            if t > result {
                result = t;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut matrix: Vec<Vec<i32>> = Vec::new();
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
                    matrix.push(tmp_row);
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
        "Max equal rows after flips: {}",
        Solution::max_equal_rows_after_flips(matrix)
    );
}
