use std::env;

struct Solution {}

impl Solution {
    fn generate_vector(s: String) -> Vec<String> {
        let mut tmp: String = String::new();
        let mut res: Vec<String> = Vec::new();
        for c in s.chars() {
            if c == ' ' {
                res.push(tmp);
                tmp = String::new();
            } else {
                tmp.push(c);
            }
        }
        if !tmp.is_empty() {
            res.push(tmp);
        }
        res
    }

    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let vec1: Vec<String> = Solution::generate_vector(sentence1);
        let len1: usize = vec1.len();
        let vec2: Vec<String> = Solution::generate_vector(sentence2);
        let len2: usize = vec2.len();
        let (short, short_len, long, long_len) = if len1 > len2 {
            (vec2, len2, vec1, len1)
        } else {
            (vec1, len1, vec2, len2)
        };
        let mut left1: usize = 0;
        let mut left2: usize = 0;
        while left1 < short_len {
            if short[left1] == long[left2] {
                left1 += 1;
                left2 += 1;
            } else {
                break;
            }
        }
        if left1 == short_len {
            return true;
        }
        let mut right1: usize = short_len - 1;
        let mut right2: usize = long_len - 1;
        loop {
            if short[right1] == long[right2] {
                if right1 == 0 {
                    return true;
                } else {
                    right1 -= 1;
                    right2 -= 1;
                }
            } else {
                break;
            }
        }
        left1 == right1 + 1
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut sentence1: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => sentence1 = arg,
            _ => {
                ret += 1;
                let sentence2: String = arg;
                println!(
                    "Similar sentence: {}",
                    Solution::are_sentences_similar(sentence1, sentence2)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
