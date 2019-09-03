use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; num_people as usize];
        let mut rest: i32 = candies;
	let mut i: usize = 0;
	let mut j: i32 = 1;
	while rest > 0 {
	    if rest > j {
	        result[i] += j;
		rest -= j;
		i += 1;
		if i == num_people as usize {
		    i = 0;
		}
	    } else {
	        result[i] += rest;
		break;
	    }
	    j += 1;
	}
	result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut candies: i32 = 0;
    let mut num_people: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            candies = i32::from_str(&arg).expect("Error parse.");
	} else if 2 == index {
            ret += 1;
            num_people = i32::from_str(&arg).expect("Error parse.");
	    break;
        } else {
	}
    }

    if 2 != ret {
        println!("Require at least two parameters.");
        return;
    }

    let result: Vec<i32> = Solution::distribute_candies(candies, num_people);
    for n in result {
        print!("{} ", n);
    }
    print!("\n");
}
