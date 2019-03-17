use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut str1: String = String::with_capacity(1024);
        str1.push(49 as char);
        let mut str2: String = String::with_capacity(1024);

	let mut use_str1: bool = true;
	for _i in 1..n {
	    let (src_str, dst_str) = match use_str1 {
	        true => (&mut str1, &mut str2),
		false => (&mut str2, &mut str1),
	    };

	    dst_str.clear();
	    let mut count_of_n: u8 = 0;
	    let mut prefix_n: u8 = match src_str.chars().next() {
	        Some(x) => x as u8,
		None => 0,
	    };
	    for number in src_str.bytes() {
	        if number == prefix_n {
		    count_of_n += 1
		} else {
		    dst_str.push((count_of_n + 48) as char);
		    dst_str.push(prefix_n as char);
		    count_of_n = 1;
		    prefix_n = number;
		}
	    }
	    dst_str.push((count_of_n + 48) as char);
	    dst_str.push(prefix_n as char);
	    use_str1 = !use_str1;
	}

	match use_str1 {
	    true => return str1,
	    false => return str2,
	}
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if index == 1 {
            ret = index;
            let n: i32  = i32::from_str(&arg).expect("Error parse.");
            println!("String: {}", Solution::count_and_say(n));
	    break;
	}
    }

    match ret {
        0 => println!("Require at least one parameter."),
	_ => (),
    }
}
