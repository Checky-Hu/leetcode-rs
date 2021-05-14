use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut population: Vec<i32> = vec![0; 101];
        for log in logs.iter() {
            for year in log[0]..log[1] {
                population[(year - 1950) as usize] += 1;
            }
        }
        // (year, population)
        let mut result: (i32, i32) = (1949, -1);
        for i in 1950..=2050 {
            if population[(i - 1950) as usize] > result.1 {
                result.0 = i;
                result.1 = population[(i - 1950) as usize];
            }
        }
        result.0
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut logs: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(num);
                if tmp.len() == 2 {
                    logs.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!(
        "Maximum population year: {}",
        Solution::maximum_population(logs)
    );
}
