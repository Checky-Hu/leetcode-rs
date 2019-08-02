use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut flags: HashMap<i32, i32> = HashMap::new();
	for n in deck {
	    if flags.contains_key(&n) {
	        *flags.get_mut(&n).unwrap() += 1;
	    } else {
	        flags.insert(n, 1);
	    }
	}
	let mut pre_count: i32 = -1;
	for val in flags.values() {
	    if pre_count < 0 {
	        pre_count = *val;
	    } else {
	        let (mut t1, mut t2) = if pre_count > *val {
		    (pre_count, *val)
		} else {
		    (*val, pre_count)
		};
		while t2 > 1 {
		    let tmp: i32 = t1 % t2;
		    if tmp == 0 {
		        break;
		    } else {
		        t1 = t2;
			t2 = tmp;
		    }
		}
		if t2 > 1 {
		    pre_count = t2;
		} else {
		    return false
		}
	    }
	}
	pre_count > 1
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut deck: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
	if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
	    deck.push(n);
	}
    }

    if 0 == ret {
        println!("Require at least one parameter.");
	return;
    }

    println!("Groups of size x: {}", Solution::has_groups_size_x(deck));
}

