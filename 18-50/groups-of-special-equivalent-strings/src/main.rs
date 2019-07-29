use std::env;

struct Solution {
}

impl Solution {
    pub fn num_special_equiv_groups(a: Vec<String>) -> i32 {
        let mut result: Vec<Vec<Vec<i32>>> = Vec::new();
	for s in a {
	    let mut odd: Vec<i32> = vec![0; 26];
	    let mut even: Vec<i32> = vec![0; 26];
	    let mut i: usize = 0;
	    for c in s.chars() {
	        if i % 2 == 0 {
		    even[(c as u8 - 97) as usize] += 1;
		} else {
		    odd[(c as u8 - 97) as usize] += 1;
		}
		i += 1;
	    }

	    let mut is_equiv: bool = false;
	    for r in &result {
	        let mut j: usize = 0;
		while j < 26 {
		    if r[0][j] != even[j] || r[1][j] != odd[j] {
		        break;
		    } else {
		        j += 1;
		    }
		}
		if j == 26 {
		    is_equiv = true;
		    break;
		}
	    }
	    if !is_equiv {
	        let mut tmp: Vec<Vec<i32>> = Vec::new();
		tmp.push(even);
		tmp.push(odd);
		result.push(tmp);
	    }
	}
	result.len() as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	if 0 != index {
            ret += 1;
            let s: String = arg;
	    a.push(s);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Groups: {}", Solution::num_special_equiv_groups(a));
}

