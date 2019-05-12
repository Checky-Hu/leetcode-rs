use std::env;
use std::str::FromStr;

struct NumMatrix {
    sum_: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut sum: Vec<Vec<i32>> = Vec::new();
	for v in matrix {
	    let mut tmp: Vec<i32> = vec![0; v.len() + 1];
	    let mut index: usize = 1;
	    for n in v {
	        tmp[index] = tmp[index - 1] + n;
	        index += 1;
	    }
	    sum.push(tmp);
	}
        NumMatrix {
	    sum_: sum,
	}
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut result: i32 = 0;
	for index in row1..=row2 {
            result += self.sum_[index as usize][col2 as usize + 1] - self.sum_[index as usize][col1 as usize]
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut row1: i32 = 0;
    let mut col1: i32 = 0;
    let mut row2: i32 = 0;
    let mut col2: i32 = 0;
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut tmp: Vec<i32> = Vec::new();
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => row1 = i32::from_str(&arg).expect("Error parse."),
	    2 => col1 = i32::from_str(&arg).expect("Error parse."),
	    3 => row2 = i32::from_str(&arg).expect("Error parse."),
	    4 => col2 = i32::from_str(&arg).expect("Error parse."),
	    5 => row = i32::from_str(&arg).expect("Error parse."),
	    6 => col = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
	        tmp.push(number);
		if ret % col as usize == 0 {
		    matrix.push(tmp);
		    tmp = Vec::new();
		}
	    },
	}
    }

    if 0 == ret || row * col != ret as i32 {
        println!("Require at least (row * col) parameters.");
	return;
    }

    let obj: NumMatrix = NumMatrix::new(matrix);
    println!("Sum: {}", obj.sum_region(row1, col1, row2, col2));
}
