use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, v) in manager.iter().enumerate() {
            if *v >= 0 {
                match map.get_mut(&(*v as usize)) {
                    Some(x) => {
                        x.push(i);
                    }
                    None => {
                        map.insert(*v as usize, vec![i]);
                    }
                }
            }
        }
        let mut result: Vec<i32> = Vec::new();
        // (id, time)
        let mut queue: Vec<(usize, i32)> = vec![(head_id as usize, 0)];
        while !queue.is_empty() {
            let mut next: Vec<(usize, i32)> = Vec::new();
            for q in queue {
                match map.get(&q.0) {
                    Some(x) => {
                        for v in x.iter() {
                            next.push((*v as usize, q.1 + inform_time[q.0]));
                        }
                    }
                    None => result.push(q.1),
                }
            }
            queue = next;
        }
        let mut max: i32 = 0;
        for r in result.iter() {
            if *r > max {
                max = *r;
            }
        }
        max
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut head_id: i32 = 0;
    let mut manager: Vec<i32> = Vec::new();
    let mut inform_time: Vec<i32> = Vec::new();
    for (i, arg) in env::args().enumerate() {
        match i {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            2 => head_id = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                if manager.len() == n as usize {
                    inform_time.push(t);
                } else {
                    manager.push(t);
                }
            }
        }
    }

    if 0 == n || 2 * n != ret {
        println!("Require at least (2 + 2 * arg1) parameters.");
        return;
    }

    println!(
        "Time needed to inform all employees: {}",
        Solution::num_of_minutes(n, head_id, manager, inform_time)
    );
}
