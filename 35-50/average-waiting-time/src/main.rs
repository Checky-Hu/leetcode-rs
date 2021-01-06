use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut num: f64 = 0_f64;
        let mut sum: f64 = 0_f64;
        let mut tmp: f64 = 0_f64;
        for v in customers.iter() {
            num += 1_f64;
            let (arrival, time) = (f64::from(v[0]), f64::from(v[1]));
            if tmp < arrival {
                tmp = arrival + time;
            } else {
                tmp += time;
            }
            sum += tmp - arrival;
        }
        sum / num
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut tmp: Vec<i32> = Vec::new();
    let mut customers: Vec<Vec<i32>> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(n);
                if tmp.len() == 2 {
                    customers.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 2 > ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!(
        "Average waiting time: {}",
        Solution::average_waiting_time(customers)
    );
}
