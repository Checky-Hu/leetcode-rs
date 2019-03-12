use std::env;

struct Solution {
}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
	let mut result: Vec<Vec<String>> = Vec::new();
	let mut flags: Vec<Vec<i32>> = Vec::new();
	for str in strs {
	    let mut flag: Vec<i32> = vec![0; 26];
	    for c in str.chars() {
	        flag[c as usize - 97] += 1;
	    }

	    let mut has_anagrams: bool = false;
	    let mut index: usize = 0;
	    for f in &flags {
	        let mut i: usize = 0;
		while i < 26 {
		    if f[i] != flag[i] {
		        break;
		    }
		    i += 1;
		}
		if i == 26 {
		    has_anagrams = true;
		    break;
		}
		index += 1;
	    }
	    if has_anagrams {
	        result[index].push(str);
	    } else {
	        flags.push(flag);
		let mut tmp_v: Vec<String> = Vec::new();
		tmp_v.push(str);
		result.push(tmp_v);
	    }
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut strs: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
	    strs.push(arg);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    let result = Solution::group_anagrams(strs);
    for v in result {
        for str in v {
            print!("{} ", str);
	}
	print!("\n");
    }
}
