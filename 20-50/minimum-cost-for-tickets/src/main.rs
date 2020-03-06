use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn helper(day: usize, costs: &[i32], memory: &mut Vec<i32>, visits: &[bool]) -> i32 {
        if day > 365 {
            return 0;
        }
        if memory[day] >= 0 {
            return memory[day];
        }
        let answer = if visits[day] {
            let t1: i32 = costs[0] + Solution::helper(day + 1, costs, memory, visits);
            let t2: i32 = costs[1] + Solution::helper(day + 7, costs, memory, visits);
            let t3: i32 = costs[2] + Solution::helper(day + 30, costs, memory, visits);
            if t1 >= t2 {
                if t2 >= t3 {
                    t3
                } else {
                    t2
                }
            } else if t1 >= t3 {
                t3
            } else {
                t1
            }
        } else {
            Solution::helper(day + 1, costs, memory, visits)
        };
        memory[day] = answer;
        answer
    }

    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut memory: Vec<i32> = vec![-1; 366];
        let mut visits: Vec<bool> = vec![false; 366];
        for day in days.iter() {
            visits[*day as usize] = true;
        }
        Solution::helper(1, &costs, &mut memory, &visits)
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut days: Vec<i32> = Vec::new();
    let mut costs: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                if costs.len() == 3 {
                    days.push(n);
                } else {
                    costs.push(n);
                }
            }
        }
    }

    if ret <= 3 {
        println!("Require at least four parameters.");
        return;
    }

    println!(
        "Min cost tickets: {}",
        Solution::mincost_tickets(days, costs)
    );
}
