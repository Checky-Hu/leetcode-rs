use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn three_sum_multi(a: Vec<i32>, target: i32) -> i32 {
        let mut counts: Vec<i32> = vec![0; 101];
        let mut vector: Vec<i32> = Vec::new();
        for v in a.iter() {
            if counts[*v as usize] == 0 {
                vector.push(*v);
            }
            counts[*v as usize] += 1;
        }
        let mut result: i32 = 0;
        let len: usize = vector.len();
        vector.sort();
        if len >= 3 {
            for i in 0..(len - 2) {
                for j in (i + 1)..(len - 1) {
                    let t: i32 = target - vector[i] - vector[j];
                    if 0 <= t && t <= 100 && t > vector[j] && counts[t as usize] > 0 {
                        result = (result
                            + counts[vector[i] as usize]
                                * counts[vector[j] as usize]
                                * counts[t as usize])
                            % 1_000_000_007;
                    }
                }
            }
        }
        if len >= 2 {
            for i in 0..len {
                if counts[vector[i] as usize] < 2 {
                    continue;
                }
                let t: i32 = target - 2 * vector[i];
                if 0 <= t && t <= 100 && t != vector[i] && counts[t as usize] > 0 {
                    result = (result
                        + counts[vector[i] as usize] * (counts[vector[i] as usize] - 1) / 2
                            * counts[t as usize])
                        % 1_000_000_007;
                }
            }
        }
        if target % 3 == 0 {
            let t: i32 = target / 3;
            let mut n: i32 = counts[t as usize];
            while n > 2 {
                result = (result + (n - 1) * (n - 2) / 2) % 1_000_000_007;
                n -= 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut target: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => target = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                a.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!(
        "Number of three sum to target: {}",
        Solution::three_sum_multi(a, target)
    );
}
