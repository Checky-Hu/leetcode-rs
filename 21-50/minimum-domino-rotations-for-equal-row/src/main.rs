use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_domino_rotations(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let len: usize = a.len();
        // (count, rotation)
        let mut record_a: Vec<(usize, i32)> = vec![(0, 0); 6];
        let mut record_b: Vec<(usize, i32)> = vec![(0, 0); 6];
        for i in 0..len {
            record_a[a[i] as usize - 1].0 += 1;
            record_b[b[i] as usize - 1].0 += 1;
            if b[i] != a[i] {
                record_a[b[i] as usize - 1].0 += 1;
                record_a[b[i] as usize - 1].1 += 1;
                record_b[a[i] as usize - 1].0 += 1;
                record_b[a[i] as usize - 1].1 += 1;
            }
        }
        let mut result: i32 = -1;
        for i in 0..6 {
            if record_a[i].0 == len && (result == -1 || record_a[i].1 < result) {
                result = record_a[i].1;
            }
            if record_b[i].0 == len && (result == -1 || record_b[i].1 < result) {
                result = record_b[i].1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                if a.len() == n as usize {
                    b.push(t);
                } else {
                    a.push(t);
                }
            }
        }
    }

    if 0 == n || 2 * n > ret {
        println!("Require at least (1 + 2 * arg1) parameters.");
        return;
    }

    println!(
        "Minimum domino rotations: {}",
        Solution::min_domino_rotations(a, b)
    );
}
