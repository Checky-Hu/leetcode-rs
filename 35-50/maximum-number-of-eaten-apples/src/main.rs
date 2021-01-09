use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let len: usize = apples.len();
        // (apple, start, end)
        let mut vec: Vec<(i32, usize, usize)> = Vec::with_capacity(len);
        for i in 0..len {
            if apples[i] != 0 {
                vec.push((apples[i], i, i + days[i] as usize));
            }
        }
        vec.sort_by(|a, b| match a.2.cmp(&b.2) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.1.cmp(&b.1),
            Ordering::Greater => Ordering::Greater,
        });
        let mut result: i32 = 0;
        if let Some(x) = vec.last() {
            let mut visit: Vec<bool> = vec![false; x.2];
            for v in vec.iter() {
                let mut count: i32 = 0;
                for j in v.1..v.2 {
                    if !visit[j] {
                        visit[j] = true;
                        count += 1;
                        if count == v.0 {
                            break;
                        }
                    }
                }
            }
            for b in visit.iter() {
                if *b {
                    result += 1;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut len: usize = 0;
    let mut apples: Vec<i32> = Vec::new();
    let mut days: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => len = usize::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                if apples.len() == len {
                    days.push(n);
                } else {
                    apples.push(n);
                }
            }
        }
    }

    if 0 == ret || len == 0 || ret != len * 2 {
        println!("Require at least (1 + 2 * arg1) parameters.");
        return;
    }

    println!(
        "Maximum number of eaten apples: {}",
        Solution::eaten_apples(apples, days)
    );
}
