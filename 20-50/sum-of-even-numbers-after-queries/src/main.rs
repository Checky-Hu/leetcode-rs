use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn sum_even_after_queries(a: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let len: usize = a.len();
        let mut flags: Vec<bool> = vec![false; len];
        let mut value: Vec<i32> = a.clone();
        let mut sum: i32 = 0;
        for i in 0..len {
            if value[i] & 1 == 0 {
                sum += value[i];
                flags[i] = true;
            }
        }

        let mut result: Vec<i32> = Vec::new();
        for query in queries {
            let tmp: i32 = value[query[1] as usize];
            value[query[1] as usize] += query[0];
            if flags[query[1] as usize] {
                if query[0] & 1 == 0 {
                    sum += query[0];
                } else {
                    sum -= tmp;
                    flags[query[1] as usize] = false;
                }
            } else {
                if query[0] & 1 == 1 {
                    sum += tmp + query[0];
                    flags[query[1] as usize] = true;
                }
            }
            result.push(sum);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a_len: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    let mut queries: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => a_len = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                if ret <= a_len {
                    a.push(number);
                } else {
                    tmp.push(number);
                    if tmp.len() == 2 {
                        queries.push(tmp);
                        tmp = Vec::new();
                    }
                }
            },
        }
    }

    if 0 == ret || 0 == a_len || queries.len() == 0 {
        println!("Require at least four parameters.");
        return;
    }

    let result: Vec<i32> = Solution::sum_even_after_queries(a, queries);
    for n in result {
        print!("{} ", n);
    }
    print!("\n");
}
