use std::env;

struct Solution {
}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut value_map: Vec<u8> = vec![0; 255];
        let mut value_count: Vec<u8> = vec![0; 255];
	let s_bytes: &[u8] = s.as_bytes();
	let t_bytes: &[u8] = t.as_bytes();
	let mut i: usize = 0;
	while i < s_bytes.len() {
	    let pos_in_map: usize = s_bytes[i] as usize;
	    if value_map[pos_in_map] == 0 {
	        let pos_in_count: usize = t_bytes[i] as usize;
	        if value_count[pos_in_count] == 0 {
		    value_count[pos_in_count] = 1;
	            value_map[pos_in_map] = t_bytes[i];
		} else {
		    break;
		}
	    } else {
	        if value_map[pos_in_map] != t_bytes[i] {
		    break;
		}
	    }
	    i += 1;
	}
	i == s.len()
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut s: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    1 => s = arg,
	    2 => {
                ret = index;
                let t: String = arg;
		println!("Isomorphic: {}", Solution::is_isomorphic(s, t));
		break;
	    },
	    _ => (),
	}
    }

    if 0 == ret {
        println!("Require at least two parameter.");
    }
}
