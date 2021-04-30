use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let len: usize = tasks.len();
        // (enqueue time, process time, index)
        let mut infos: Vec<(i32, i32, i32)> = Vec::new();
        for (i, task) in tasks.iter().enumerate() {
            infos.push((task[0], task[1], i as i32));
        }
        infos.sort_by(|a, b| match b.0.cmp(&a.0) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => match b.1.cmp(&a.1) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => b.2.cmp(&a.2),
                Ordering::Greater => Ordering::Greater,
            },
            Ordering::Greater => Ordering::Greater,
        });
        let mut result: Vec<i32> = Vec::new();
        let mut time: i32 = 0;
        let mut bucket: Vec<(i32, i32)> = Vec::new();
        loop {
            while let Some(x) = infos.last() {
                if bucket.is_empty() {
                    if x.0 > time {
                        time = x.0;
                    }
                    bucket.push((x.1, x.2));
                    infos.pop();
                } else if x.0 <= time {
                    let mut index: usize = 0;
                    for v in bucket.iter() {
                        if v.0 < x.1 || (v.0 == x.1 && v.1 < x.2) {
                            break;
                        } else {
                            index += 1;
                        }
                    }
                    bucket.insert(index, (x.1, x.2));
                    infos.pop();
                } else {
                    break;
                }
            }
            if let Some(x) = bucket.pop() {
                time += x.0;
                result.push(x.1);
                if result.len() == len {
                    break;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut tasks: Vec<Vec<i32>> = Vec::new();
    let mut task: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let time: i32 = i32::from_str(&arg).expect("Error parse.");
                task.push(time);
                if task.len() == 2 {
                    tasks.push(task);
                    task = Vec::new();
                }
            }
        }
    }

    if 2 > ret {
        println!("Require at least 2 parameters.");
    }

    let result = Solution::get_order(tasks);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
