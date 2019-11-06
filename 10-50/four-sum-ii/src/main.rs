use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
        let mut map1: HashMap<i32, i32> = HashMap::new();
        let mut map2: HashMap<i32, i32> = HashMap::new();
        let len: usize = a.len();
        for i in 0..len {
            for j in 0..len {
                let mut t: i32 = a[i] + b[j];
                match map1.get_mut(&t) {
                    Some(x) => *x += 1,
                    None => { map1.insert(t, 1); },
                }
                t = c[i] + d[j];
                match map2.get_mut(&t) {
                    Some(x) => *x += 1,
                    None => { map2.insert(t, 1); },
                }
            }
        }
        let mut result: i32 = 0;
        for (k, v) in map1.iter() {
            result += match map2.get(&(0 - k)) {
                Some(x) => *x * v,
                None => 0,
            };
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();
    let mut c: Vec<i32> = Vec::new();
    let mut d: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                if ret <= n {
                    a.push(num);
                } else if ret <= 2 * n {
                    b.push(num);
                } else if ret <= 3 * n {
                    c.push(num);
                } else {
                    d.push(num);
                }
            },
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }

    println!("Four sum count: {}", Solution::four_sum_count(a, b, c, d));
}
