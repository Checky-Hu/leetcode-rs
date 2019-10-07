use std::env;

struct Solution {
}

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut flags: Vec<Vec<i32>> = Vec::new();
        for w in words {
            let mut t: i32 = 0;
            for c in w.chars() {
                t |= 1 << (c as u8 - 97);
            }
            flags.push(vec![t, w.len() as i32]);
        }
        let mut result: i32 = 0;
        let len: usize = flags.len();
        let mut i: usize = 0;
        while i < len {
            let mut j: usize = i + 1;
            while j < len {
                if flags[i][0] & flags[j][0] == 0 {
                    let t: i32 = flags[i][1] * flags[j][1];
                    if t > result {
                        result = t;
                    }
                }
                j += 1;
            }
            i += 1;
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut words: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let word: String = arg;
                words.push(word);
            },
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Max product: {}", Solution::max_product(words));
}
