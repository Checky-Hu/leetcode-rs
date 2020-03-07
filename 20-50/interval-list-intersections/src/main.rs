use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn interval_intersection(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let a_len: usize = a.len();
        let b_len: usize = b.len();
        let mut a_i: usize = 0;
        let mut b_i: usize = 0;
        let mut result: Vec<Vec<i32>> = Vec::new();
        while a_i < a_len && b_i < b_len {
            if a[a_i][1] < b[b_i][0] {
                a_i += 1;
            } else if a[a_i][0] > b[b_i][1] {
                b_i += 1;
            } else {
                let start: i32 = if a[a_i][0] <= b[b_i][0] {
                    b[b_i][0]
                } else {
                    a[a_i][0]
                };
                match a[a_i][1].cmp(&b[b_i][1]) {
                    Ordering::Less => {
                        result.push(vec![start, a[a_i][1]]);
                        a_i += 1;
                    }
                    Ordering::Equal => {
                        result.push(vec![start, a[a_i][1]]);
                        a_i += 1;
                        b_i += 1;
                    }
                    Ordering::Greater => {
                        result.push(vec![start, b[b_i][1]]);
                        b_i += 1;
                    }
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut a: Vec<Vec<i32>> = Vec::new();
    let mut b: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(number);
                if tmp_row.len() == 2 {
                    if a.len() == n as usize {
                        b.push(tmp_row);
                    } else {
                        a.push(tmp_row);
                    }
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if ret == 0 {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<Vec<i32>> = Solution::interval_intersection(a, b);
    for r in result.iter() {
        print!("[{}, {}] ", r[0], r[1]);
    }
    println!();
}
