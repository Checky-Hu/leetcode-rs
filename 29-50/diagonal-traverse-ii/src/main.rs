use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        // (row + column, row, value)
        let mut counts: Vec<(usize, usize, i32)> = Vec::new();
        for (i, row) in nums.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                counts.push((i + j, i, *c));
            }
        }
        counts.sort_by(|a, b| match a.0.cmp(&b.0) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => b.1.cmp(&a.1),
            Ordering::Greater => Ordering::Greater,
        });
        let mut result: Vec<i32> = Vec::new();
        for count in counts.iter() {
            result.push(count.2);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut nums: Vec<Vec<i32>> = Vec::new();
    let mut c: i32 = 0;
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                if c == 0 {
                    c = number;
                } else {
                    tmp_row.push(number);
                    if c as usize == tmp_row.len() {
                        nums.push(tmp_row);
                        tmp_row = Vec::new();
                        c = 0;
                    }
                }
            }
        }
    }

    if 0 == ret || c != 0 {
        println!("Require at least 2 parameters.");
        return;
    }

    let result: Vec<i32> = Solution::find_diagonal_order(nums);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
