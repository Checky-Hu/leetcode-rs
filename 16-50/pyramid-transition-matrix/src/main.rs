use std::collections::HashMap;
use std::env;

struct Solution {}

impl Solution {
    fn transition_loop(bottom: &[u8], map: &HashMap<(u8, u8), Vec<u8>>) -> bool {
        let len: usize = bottom.len();
        if len == 2 {
            map.contains_key(&(bottom[0], bottom[1]))
        } else {
            let mut nexts: Vec<Vec<u8>> = Vec::new();
            for i in 0..(len - 1) {
                match map.get(&(bottom[i], bottom[i + 1])) {
                    Some(x) => {
                        let mut cur: Vec<Vec<u8>> = Vec::new();
                        if i == 0 {
                            for u in x {
                                cur.push(vec![*u]);
                            }
                        } else {
                            for next in nexts {
                                for u in x {
                                    let mut tmp: Vec<u8> = next.clone();
                                    tmp.push(*u);
                                    cur.push(tmp);
                                }
                            }
                        }
                        nexts = cur;
                    }
                    None => return false,
                }
            }
            for next in &nexts {
                if Solution::transition_loop(next, map) {
                    return true;
                }
            }
            false
        }
    }

    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let mut map: HashMap<(u8, u8), Vec<u8>> = HashMap::new();
        for s in allowed {
            let bytes: &[u8] = s.as_bytes();
            match map.get_mut(&(bytes[0], bytes[1])) {
                Some(x) => (*x).push(bytes[2]),
                None => {
                    map.insert((bytes[0], bytes[1]), vec![bytes[2]]);
                }
            }
        }
        Solution::transition_loop(bottom.as_bytes(), &map)
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut bottom: String = String::new();
    let mut allowed: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => bottom = arg,
            _ => {
                ret += 1;
                let s: String = arg;
                allowed.push(s);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!(
        "Transition pyramid: {}",
        Solution::pyramid_transition(bottom, allowed)
    );
}
