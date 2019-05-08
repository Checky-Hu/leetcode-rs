use std::env;

struct Solution {
}

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
	let s_bytes: &[u8] = secret.as_bytes();
	let g_bytes: &[u8] = guess.as_bytes();
	let mut s_count: Vec<i32> = vec![0; 10];
	let mut g_count: Vec<i32> = vec![0; 10];
	let mut a_count: i32 = 0;
	let mut i: usize = 0;
	while i < s_bytes.len() {
	    if s_bytes[i] == g_bytes[i] {
	        a_count += 1;
	    } else {
	        s_count[s_bytes[i] as usize - 48] += 1;
	        g_count[g_bytes[i] as usize - 48] += 1;
	    }
	    i += 1;
	}
	let mut b_count: i32 = 0;
	i = 0;
	while i < 10 {
	    b_count += if s_count[i] < g_count[i] {
	        s_count[i]
	    } else {
	        g_count[i]
	    };
	    i += 1;
	}

        let mut result: String = String::new();
	result.push_str(&a_count.to_string());
	result.push('A');
	result.push_str(&b_count.to_string());
	result.push('B');
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut secret: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    1 => secret = arg,
	    2 => {
                ret = index;
                let guess: String = arg;
                println!("Hint: {}", Solution::get_hint(secret, guess));
		break;
	    },
	    _ => (),
	}
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
