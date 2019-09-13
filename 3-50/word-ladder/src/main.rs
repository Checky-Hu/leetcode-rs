use std::collections::HashSet;
use std::env;

struct Solution {
}

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut word_set: HashSet<String> = word_list.into_iter().collect();
	if !word_set.contains(&end_word) {
	    return 0
	}
	let mut queue: Vec<String> = vec![begin_word];
	let mut result: i32 = 0;
	while !queue.is_empty() {
	    let q_len: usize = queue.len();
	    for _i in 0..q_len {
	        let mut word: String = queue.remove(0);
		if word == end_word {
		    return result + 1
		} else {
		    let w_len: usize = word.len();
		    for j in 0..w_len {
		        unsafe {
		            let new_word: &mut Vec<u8> = word.as_mut_vec();
			    let original_c: u8 = new_word[j];
			    let mut c: u8 = 97;
			    while c <= 122 {
			        new_word[j] = c;
			        let tmp: String = String::from_utf8(new_word.to_vec()).unwrap();
			        if word_set.contains(&tmp) {
			            queue.push(tmp.clone());
				    word_set.remove(&tmp);
			        }
			        c += 1;
			    }
			    new_word[j] = original_c;
			}
		    }
		}
	    }
	    result += 1;
	}
	0
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut begin_word: String = String::new();
    let mut end_word: String = String::new();
    let mut word_list: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => begin_word = arg,
	    2 => end_word = arg,
	    _ => {
                ret += 1;
                let s: String = arg;
		word_list.push(s);
	    },
	}
    }

    if ret == 0 {
        println!("Require at least 3 parameters.");
	return;
    }

    println!("Ladder length: {}", Solution::ladder_length(begin_word, end_word, word_list));
}
