use std::env;

struct Solution {
}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut num_map: Vec<Vec<char>> = Vec::new();
        num_map.push(vec!['a', 'b', 'c']);
        num_map.push(vec!['d', 'e', 'f']);
        num_map.push(vec!['g', 'h', 'i']);
        num_map.push(vec!['j', 'k', 'l']);
        num_map.push(vec!['m', 'n', 'o']);
        num_map.push(vec!['p', 'q', 'r', 's']);
        num_map.push(vec!['t', 'u', 'v']);
        num_map.push(vec!['w', 'x', 'y', 'z']);

	for n in digits.chars() {
	    let cur_index: usize = n as usize - 50;
	    let tmp_len: usize = num_map[cur_index].len();
	    if result.is_empty() {
	        for i in 0..tmp_len {
		    result.push(num_map[cur_index][i].to_string());
		}
	    } else {
	        let mut tmp_vec: Vec<String> = Vec::new();
	        while !result.is_empty() {
		    tmp_vec.push(result.pop().unwrap());
		}
		for str in tmp_vec {
		    for i in 0..tmp_len {
		        let mut tmp_str: String = String::new();
			tmp_str.push_str(&str);
			tmp_str.push(num_map[cur_index][i]);
		        result.push(tmp_str);
		    }
		}
	    }
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let digits: String = arg;
            let result: Vec<String> = Solution::letter_combinations(digits);
            for str in result {
                println!("{} ", str);
	    }
	    break;
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
