use std::env;
use std::str::FromStr;

struct NumArray {
    sum_: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sum: Vec<i32> = vec![0; nums.len() + 1];
	let mut index: usize = 1;
	while index <= nums.len() {
	    sum[index] = sum[index - 1] + nums[index - 1];
	    index += 1;
	}
        NumArray {
	    sum_: sum,
	}
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.sum_[j as usize + 1] - self.sum_[i as usize]
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => i = i32::from_str(&arg).expect("Error parse."),
	    2 => j = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret = index;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        nums.push(number);
	    },
	}
    }

    if 0 == ret {
        println!("Require at least three parameters.");
	return;
    }

    let obj: NumArray = NumArray::new(nums);
    println!("Sum: {}", obj.sum_range(i, j));
}
