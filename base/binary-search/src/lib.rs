#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod bsi32 {
    pub fn binary_search(nums: &Vec<i32>, left: usize, right: usize, target: i32) -> i32 {
	let mut i: usize = left;
	let mut j: usize = right;
	while i <= j {
	    let index: usize = i + (j - i) / 2;
	    if nums[index] == target {
	        return index as i32
	    } else if nums[index] < target {
	        i = index + 1;
	    } else {
	        if index >= 1 {
	            j = index - 1;
		} else {
		    break;
		}
	    }
	}
	-1
    }
}
