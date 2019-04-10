use std::env;

struct Solution {
}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut tmp_paths: Vec<String> = Vec::new();
	let path_bytes: &[u8] = path.as_bytes();
	let len: usize = path_bytes.len();
	let mut i: usize = 0;
	while i < len {
	    while i < len && path_bytes[i] == 47 {
	        i += 1;
	    }
	    if i == len {
	        break;
	    }
	    let start: usize = i;

	    while i < len && path_bytes[i] != 47 {
	        i += 1;
	    }
	    let end: usize = i - 1;

	    let mut tmp_path: String = String::new();
	    for tmp in start..=end {
	        tmp_path.push(path_bytes[tmp] as char);
	    }
	    if tmp_path == "." {
	        continue;
	    } else if tmp_path == ".." {
	        tmp_paths.pop();
	    } else {
	        tmp_paths.push(tmp_path);
	    }
	}

	if tmp_paths.is_empty() {
	    "/".to_string()
	} else {
	    let mut result: String = String::new();
	    for path in tmp_paths {
	        result.push('/');
		result.push_str(path.as_str());
	    }
	    result
	}
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let path: String = arg;
            println!("Path: {}", Solution::simplify_path(path));
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
