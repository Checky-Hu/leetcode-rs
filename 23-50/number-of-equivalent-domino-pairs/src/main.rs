use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        // There are 45 conditions.
        let mut flags: Vec<i32> = vec![0; 45];
        for v in dominoes {
            let (s, b) = if v[0] > v[1] {
                (v[1] as usize, v[0] as usize)
            } else {
                (v[0] as usize, v[1] as usize)
            };
            flags[(18 - s) * (s - 1) / 2 + b - 1] += 1;
        }
        let mut result: i32 = 0;
        for n in flags {
            result += (n - 1) * n / 2;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut dominoes: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(n);
                if ret & 1 == 0 {
                    dominoes.push(tmp);
                    tmp = Vec::new();
                }
            },
        }
    }

    if 0 == ret || ret & 1 != 0 {
        println!("Require at least (2*n) parameters.");
        return;
    }

    println!("Pairs: {}", Solution::num_equiv_domino_pairs(dominoes));
}
