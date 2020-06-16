use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let len: usize = arr.len();
        let mut xor: Vec<i32> = Vec::with_capacity(len as usize);
        for i in 0..len {
            xor.push(if i == 0 { arr[i] } else { xor[i - 1] ^ arr[i] });
        }
        let mut result: Vec<i32> = Vec::with_capacity(len as usize);
        for query in queries.iter() {
            result.push(if query[0] > 0 {
                xor[query[0] as usize - 1] ^ xor[query[1] as usize]
            } else {
                xor[query[1] as usize]
            });
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    let mut queries: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                if arr.len() == n as usize {
                    tmp.push(t);
                    if tmp.len() == 2 {
                        queries.push(tmp);
                        tmp = Vec::new();
                    }
                } else {
                    arr.push(t);
                }
            }
        }
    }

    if 0 == n || 3 * n != ret {
        println!("Require at least (1 + 3 * arg1) parameters.");
    }

    let result: Vec<i32> = Solution::xor_queries(arr, queries);
    for r in &result {
        print!("{} ", *r);
    }
    println!();
}
