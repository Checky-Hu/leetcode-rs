use std::env;

struct Solution {}

impl Solution {
    fn is_predecessor(a: &[u8], b: &[u8]) -> bool {
        let a_len: usize = a.len();
        let b_len: usize = b.len();
        if a_len + 1 != b_len {
            return false;
        }
        let mut meet_add: bool = false;
        let mut a_i: usize = 0;
        let mut b_i: usize = 0;
        while a_i < a_len && b_i < b_len {
            if a[a_i] == b[b_i] {
                a_i += 1;
                b_i += 1;
            } else if !meet_add {
                meet_add = true;
                b_i += 1;
            } else {
                return false;
            }
        }
        true
    }

    fn helper(words: &[String], i: usize, len: usize, memo: &mut Vec<i32>) -> i32 {
        if memo[i] == 0 {
            let mut result: i32 = 1;
            for j in (i + 1)..len {
                if Solution::is_predecessor(words[i].as_bytes(), words[j].as_bytes()) {
                    let t: i32 = Solution::helper(&words, j, len, memo) + 1;
                    if t > result {
                        result = t;
                    }
                }
            }
            memo[i] = result;
        }
        memo[i]
    }

    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let len: usize = words.len();
        let mut mut_words: Vec<String> = words;
        mut_words.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut memo: Vec<i32> = vec![0; len];
        let mut result: i32 = 0;
        for i in 0..len {
            let t: i32 = Solution::helper(&mut_words, i, len, &mut memo);
            if t > result {
                result = t;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut words: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let s: String = arg;
            words.push(s);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Longest string chain: {}",
        Solution::longest_str_chain(words)
    );
}
