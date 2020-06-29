use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut mut_events: Vec<Vec<i32>> = events;
        mut_events.sort_by(|a, b| match a[0].cmp(&b[0]) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a[1].cmp(&b[1]),
            Ordering::Greater => Ordering::Greater,
        });
        let mut result: i32 = 0;
        let len: usize = mut_events.len();
        let mut stack: Vec<i32> = Vec::new();
        let mut index: usize = 0;
        for day in 1..=100_000 {
            if stack.is_empty() && index >= len {
                break;
            }
            while index < len && mut_events[index][0] <= day {
                let mut t: usize = 0;
                for (i, v) in stack.iter().enumerate() {
                    if mut_events[index][1] >= *v {
                        t = i;
                        break;
                    } else {
                        t += 1;
                    }
                }
                stack.insert(t, mut_events[index][1]);
                index += 1;
            }
            while let Some(x) = stack.last() {
                if *x < day {
                    stack.pop();
                } else {
                    break;
                }
            }
            if !stack.is_empty() {
                stack.pop();
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut events: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(n);
                if tmp.len() == 2 {
                    events.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!("Maximum events: {}", Solution::max_events(events));
}
