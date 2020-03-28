use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut status: Vec<i32> = vec![0; 1000];
        for trip in trips.iter() {
            for i in trip[1]..trip[2] {
                status[i as usize] += trip[0];
            }
        }
        for state in status.iter() {
            if *state > capacity {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut capacity: i32 = 0;
    let mut trips: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => capacity = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(n);
                if tmp_row.len() == 3 {
                    trips.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Car pooling: {}", Solution::car_pooling(trips, capacity));
}
