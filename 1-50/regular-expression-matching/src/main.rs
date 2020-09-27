use std::collections::HashMap;
use std::env;

struct Solution {}

impl Solution {
    fn loop_match(
        s: &[u8],
        s_len: usize,
        s_index: usize,
        p: &[u8],
        p_len: usize,
        p_index: usize,
        mut mem: &mut HashMap<(usize, usize), bool>,
    ) -> bool {
        if p_index == p_len {
            return s_index == s_len;
        }
        if let Some(x) = mem.get(&(s_index, p_index)) {
            return *x;
        }
        let match_first: bool = s_index < s_len && (s[s_index] == p[p_index] || p[p_index] == b'.');
        let result: bool = if p_index + 1 < p_len && p[p_index + 1] == b'*' {
            Solution::loop_match(s, s_len, s_index, p, p_len, p_index + 2, &mut mem)
                || (match_first
                    && Solution::loop_match(s, s_len, s_index + 1, p, p_len, p_index, &mut mem))
        } else {
            match_first
                && Solution::loop_match(s, s_len, s_index + 1, p, p_len, p_index + 1, &mut mem)
        };
        mem.insert((s_index, p_index), result);
        result
    }

    pub fn is_match(s: String, p: String) -> bool {
        let mut mem: HashMap<(usize, usize), bool> = HashMap::new();
        let s_bytes: &[u8] = s.as_bytes();
        let p_bytes: &[u8] = p.as_bytes();
        Solution::loop_match(
            s_bytes,
            s_bytes.len(),
            0,
            p_bytes,
            p_bytes.len(),
            0,
            &mut mem,
        )
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut s: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            _ => {
                ret += 1;
                let p: String = arg;
                println!("Is match: {}", Solution::is_match(s, p));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
