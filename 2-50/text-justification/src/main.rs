use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        let len: usize = words.len();
        let mut i: usize = 0;
        while i < len {
            let mut width: usize = 0;
            let mut total: usize = 0;
            let mut count: usize = 0;
            let mut j: usize = i;
            while j < len {
                width += words[j].len();
                total += words[j].len();
                if count > 0 {
                    total += 1;
                }
                if total <= max_width as usize {
                    count += 1;
                    j += 1;
                } else {
                    width -= words[j].len();
                    break;
                }
            }
            let (common, index) = if count == 1 {
                (max_width as usize - width, 1)
            } else if j == len {
                (1, 0)
            } else {
                let normal: usize = (max_width as usize - width) / (count - 1);
                (normal, (max_width as usize - width) % (count - 1))
            };
            let mut tmp: String = String::new();
            for k in i..j {
                tmp.push_str(&words[k]);
                if k + 1 < j {
                    let space: usize = if k < i + index { common + 1 } else { common };
                    for _i in 0..space {
                        tmp.push(' ');
                    }
                }
            }
            while tmp.len() < max_width as usize {
                tmp.push(' ');
            }
            results.push(tmp);
            i = j;
        }
        results
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut max_width: i32 = 0;
    let mut words: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => max_width = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                words.push(arg);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    let result: Vec<String> = Solution::full_justify(words, max_width);
    for r in result.iter() {
        println!("{} ", r);
    }
}
