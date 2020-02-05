use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn largest_overlap(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> i32 {
        let len: usize = a.len();
        let mut a_1: Vec<(i32, i32)> = Vec::new();
        let mut b_1: Vec<(i32, i32)> = Vec::new();
        for i in 0..len {
            for j in 0..len {
                if a[i][j] == 1 {
                    a_1.push((i as i32, j as i32));
                }
                if b[i][j] == 1 {
                    b_1.push((i as i32, j as i32));
                }
            }
        }
        let mut map: HashMap<(i32, i32), i32> = HashMap::new();
        for v1 in &a_1 {
            for v2 in &b_1 {
                let tmp: (i32, i32) = (v1.0 - v2.0, v1.1 - v2.1);
                match map.get_mut(&tmp) {
                    Some(x) => *x += 1,
                    None => {
                        map.insert(tmp, 1);
                    }
                }
            }
        }
        let mut result: i32 = 0;
        for v in map.values() {
            if *v > result {
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
                if tmp_row.len() == n as usize {
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

    if 0 == n || 2 * n * n != ret {
        println!("Require at least (1 + arg1 ^ 2) parameters.");
        return;
    }

    println!("Largest overlap: {}", Solution::largest_overlap(a, b));
}
