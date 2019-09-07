use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn count(s: &String) -> i32 {
        let mut result: i32 = 0;
        let mut min_c: char = 'z';
        for c in s.chars() {
            if c < min_c {
                min_c = c;
                result = 1;
            } else if c == min_c {
                result += 1;
            }
        }
        result
    }

    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let w_len: usize = words.len();
        let mut w_flag: Vec<i32> = vec![0; w_len];
        for i in 0..w_len {
            w_flag[i] = Solution::count(&words[i]);
        }
        let mut result: Vec<i32> = Vec::new();
        for s in queries {
            let mut count: i32 = 0;
            let tmp: i32 = Solution::count(&s);
            for i in 0..w_len {
                if w_flag[i] > tmp {
                    count += 1;
                }
            }
            result.push(count);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut queries_len: i32 = 0;
    let mut queries: Vec<String> = Vec::new();
    let mut words: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => queries_len = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let tmp: String = arg;
                if ret <= queries_len {
                    queries.push(tmp);
                } else {
                    words.push(tmp);
                }
            },
        }
    }

    if 0 == queries_len || queries_len >= ret {
        println!("Require at least (queries_len + 1) parameters.");
        return;
    }

    let result: Vec<i32> = Solution::num_smaller_by_frequency(queries, words);
    for n in result {
        print!("{} ", n);
    }
    print!("\n");
}
