use std::env;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    #[inline]
    pub fn new(start: i32, end: i32) -> Self {
        Interval {
	    start,
	    end,
	}
    }
}

struct Solution {
}

impl Solution {
    pub fn merge(intervals: Vec<Interval>) -> Vec<Interval> {
        let mut result: Vec<Interval> = Vec::new();
	for i in intervals {
	    let mut index: usize = 0;
	    let mut tmp_s: i32 = i.start;
	    let mut tmp_e: i32 = i.end;
	    for j in &result {
	        if tmp_s < j.start {
		    if tmp_e < j.start {
			break;
		    } else {
			tmp_e = if tmp_e < j.end {
			    j.end
			} else {
			    tmp_e
			};
		    }
		} else {
		    if tmp_s <= j.end {
			tmp_s = j.start;
		        tmp_e = if tmp_e <= j.end {
			    j.end
			} else {
			    tmp_e
			};
		    }
		}
		index += 1;
	    }

	    let mut pos: usize = 0;
	    let mut i: usize = 0;
	    while i < index {
		if result[pos].start >= tmp_s && result[pos].end <= tmp_e {
		    result.remove(pos);
		} else {
		    pos += 1;
		}
		i += 1;
	    }

	    if pos < result.len() {
		result.insert(pos, Interval::new(tmp_s, tmp_e));
	    } else {
	        result.push(Interval::new(tmp_s, tmp_e));
	    }
	}
	result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut intervals: Vec<Interval> = Vec::new();
    let mut start: i32 = 0;
    let mut end: i32;
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
	    if index % 2 == 1 {
	        start = i32::from_str(&arg).expect("Error parse.");
	    } else {
                ret = index;
                end = i32::from_str(&arg).expect("Error parse.");
	        intervals.push(Interval::new(start, end));
	    }
	}
    }

    if 0 == ret {
        println!("Require at least two parameter.");
	return;
    }

    let result: Vec<Interval> = Solution::merge(intervals);
    for i in result {
        println!("[{},{}]", i.start, i.end);
    }
}
