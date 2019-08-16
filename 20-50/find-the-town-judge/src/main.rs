use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut flags: Vec<i32> = vec![0; n as usize];
        for v in trust {
            flags[v[0] as usize - 1] = -1;
            if flags[v[1] as usize - 1] >= 0 {
                flags[v[1] as usize - 1] += 1;
            }
        }

        let mut i: i32 = 1;
        for t in flags {
            if t == n - 1 {
                return i
            } else {
                i += 1;
            }
        }
        -1
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut trust: Vec<Vec<i32>> = Vec::new();
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
                    trust.push(tmp_row);
                    tmp_row = Vec::new();
                }
            },
        }
    }

    if 0 == ret || 0 != ret & 1 {
        println!("Require at least (2 * n + 1) parameters.");
        return;
    }

    println!("The town judge: {}", Solution::find_judge(n, trust));
}
