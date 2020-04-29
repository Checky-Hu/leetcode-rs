use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let len: usize = colsum.len();
        if len == 0 {
            return Vec::new();
        }
        let mut mut_upper: i32 = upper;
        let mut mut_lower: i32 = lower;
        let mut uncertain: i32 = 0;
        let mut visits: Vec<bool> = vec![false; len];
        let mut result: Vec<Vec<i32>> = vec![vec![0; len]; 2];
        for (i, v) in colsum.iter().enumerate() {
            match *v {
                0 => visits[i] = true,
                1 => uncertain += 1,
                _ => {
                    visits[i] = true;
                    result[0][i] = 1;
                    mut_upper -= 1;
                    result[1][i] = 1;
                    mut_lower -= 1;
                }
            }
        }
        if mut_upper < 0 || mut_lower < 0 || mut_upper + mut_lower != uncertain {
            Vec::new()
        } else {
            for (i, v) in visits.iter().enumerate() {
                if !*v {
                    if mut_upper > 0 {
                        result[0][i] = 1;
                        mut_upper -= 1;
                    } else {
                        result[1][i] = 1;
                        mut_lower -= 1;
                    }
                }
            }
            result
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut upper: i32 = 0;
    let mut lower: i32 = 0;
    let mut colsum: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => upper = i32::from_str(&arg).expect("Error parse."),
            2 => lower = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                colsum.push(number);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    let result: Vec<Vec<i32>> = Solution::reconstruct_matrix(upper, lower, colsum);
    for row in result.iter() {
        for r in row.iter() {
            print!("{} ", *r);
        }
        println!();
    }
}
