use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
        let len: usize = stones.len();
        let mut tmp: Vec<i32> = stones;
        tmp.sort();
        let mut s: usize = 0;
        let mut min: usize = len;
        for i in 0..len {
            while tmp[i] - tmp[s] >= len as i32 {
                s += 1;
            }
            if i - s == len - 2 && tmp[i] - tmp[s] == len as i32 - 2 {
                if min > 2 {
                    min = 2;
                }
            } else {
                let cur: usize = len - (i - s + 1);
                if min > cur {
                    min = cur;
                }
            }
        }
        let max1: i32 = tmp[len - 1] - tmp[1];
        let max2: i32 = tmp[len - 2] - tmp[0];
        vec![
            min as i32,
            if max1 > max2 {
                max1 + 2 - len as i32
            } else {
                max2 + 2 - len as i32
            },
        ]
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut stones: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            stones.push(n);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<i32> = Solution::num_moves_stones_ii(stones);
    println!("Min and max moves: [{}, {}]", result[0], result[1]);
}
