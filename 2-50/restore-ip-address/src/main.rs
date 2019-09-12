use std::env;

struct Solution {
}

impl Solution {
    pub fn get_next_range(s_bytes: &[u8], index: usize, count: i32, cur_res: String, mut results: &mut Vec<String>) {
        if count == 4 {
            if index == s_bytes.len() {
                results.push(cur_res);
            }
        } else {
            for i in 1..4 {
                if i + index <= s_bytes.len() {
                    let mut tmp: i32 = 0;
                    for j in 0..i {
                        tmp = tmp * 10 + s_bytes[index + j] as i32 - 48;
                    }
                    let tmp_str: String = tmp.to_string();
                    if tmp <= 255 && i == tmp_str.len() {
                        let mut result: String = cur_res.clone();
                        result.push_str(&tmp_str);
                        if count != 3 {
                            result.push('.');
                        }
                        Solution::get_next_range(s_bytes, index + i, count + 1, result, &mut results);
                    }
                } else {
                    break;
                }
            }
        }
    }

    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        Solution::get_next_range(s.as_bytes(), 0, 0, "".to_string(), &mut results);
        results
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let s: String = arg;
            let result: Vec<String> = Solution::restore_ip_addresses(s);
            for t in result {
                println!("{}", t);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
