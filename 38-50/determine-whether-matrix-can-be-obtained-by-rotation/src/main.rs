use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let mut mat_mut: Vec<Vec<i32>> = mat;
        let len: usize = target.len();
        let mut count: i32 = 0;
        let mut is_same: bool = true;
        loop {
            for (j, col) in mat_mut.iter().enumerate() {
                for (k, val) in col.iter().enumerate() {
                    if *val != target[j][k] {
                        is_same = false;
                        break;
                    }
                }
                if !is_same {
                    break;
                }
            }
            if is_same || count == 3 {
                break;
            } else {
                let mut tmp: Vec<Vec<i32>> = vec![vec![0; len]; len];
                for (j, col) in mat_mut.iter().enumerate() {
                    for (k, val) in col.iter().enumerate() {
                        tmp[k][len - 1 - j] = *val;
                    }
                }
                mat_mut = tmp;
                count += 1;
                is_same = true;
            }
        }
        is_same
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut n: usize = 0;
    let mut mat: Vec<Vec<i32>> = Vec::new();
    let mut target: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = usize::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(num);
                if tmp.len() == n {
                    if mat.len() == n {
                        target.push(tmp);
                    } else {
                        mat.push(tmp);
                    }
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == n || 2 * n * n != ret {
        println!("Require at least (1 + 2 * arg1 * arg1) parameters.");
        return;
    }

    println!("Find rotation: {}", Solution::find_rotation(mat, target));
}
