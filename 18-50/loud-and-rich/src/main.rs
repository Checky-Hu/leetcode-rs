use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let len: usize = quiet.len();
        let mut poorer: Vec<Vec<usize>> = vec![Vec::new(); len];
        let mut counts: Vec<i32> = vec![0; len];
        for r in &richer {
            poorer[r[0] as usize].push(r[1] as usize);
            counts[r[1] as usize] += 1;
        }
        let mut queue: Vec<usize> = Vec::new();
        let mut result: Vec<i32> = Vec::with_capacity(len);
        for (i, v) in counts.iter().enumerate().take(len) {
            if *v == 0 {
                queue.push(i);
            }
            result.push(i as i32);
        }
        while !queue.is_empty() {
            let mut tmp: Vec<usize> = Vec::new();
            for q in queue {
                for poor in &poorer[q] {
                    if quiet[result[q] as usize] < quiet[result[*poor] as usize] {
                        result[*poor] = result[q];
                    }
                    counts[*poor] -= 1;
                    if counts[*poor] == 0 {
                        tmp.push(*poor);
                    }
                }
            }
            queue = tmp;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut quiet: Vec<i32> = Vec::new();
    let mut richer: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let v: i32 = i32::from_str(&arg).expect("Error parse.");
                if quiet.len() as i32 == n {
                    tmp.push(v);
                    if tmp.len() == 2 {
                        richer.push(tmp);
                        tmp = Vec::new();
                    }
                } else {
                    quiet.push(v);
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least (1 + arg1) parameters.");
        return;
    }

    let result: Vec<i32> = Solution::loud_and_rich(richer, quiet);
    for r in &result {
        print!("{} ", *r);
    }
    println!();
}
