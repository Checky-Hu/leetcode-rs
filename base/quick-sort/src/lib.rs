#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod qsi32 {
    pub fn quick_sort(nums: &mut Vec<i32>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mut i: usize = left;
        let mut j: usize = right;
        let temp: i32 = nums[left];
        while i < j {
            while i < j && nums[j] >= temp {
                j -= 1;
            }
            nums[i] = nums[j];
            while i < j && nums[i] <= temp {
                i += 1;
            }
            nums[j] = nums[i];
        }
        nums[i] = temp;
        if left + 1 < i {
            quick_sort(nums, left, i - 1);
        }
        quick_sort(nums, i + 1, right);
    }

    pub fn quick_sort_descend(nums: &mut Vec<i32>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mut i: usize = left;
        let mut j: usize = right;
        let temp: i32 = nums[left];
        while i < j {
            while i < j && nums[j] <= temp {
                j -= 1;
            }
            nums[i] = nums[j];
            while i < j && nums[i] >= temp {
                i += 1;
            }
            nums[j] = nums[i];
        }
        nums[i] = temp;
        if left + 1 < i {
            quick_sort_descend(nums, left, i - 1);
        }
        quick_sort_descend(nums, i + 1, right);
    }
}

pub mod qsstr {
    pub fn quick_sort_by_descend_len(strs: &mut Vec<String>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mut i: usize = left;
        let mut j: usize = right;
        let temp: String = strs[left].clone();
        while i < j {
            while i < j && strs[j].len() <= temp.len() {
                j -= 1;
            }
            strs[i] = strs[j].clone();
            while i < j && strs[i].len() >= temp.len() {
                i += 1;
            }
            strs[j] = strs[i].clone();
        }
        strs[i] = temp;
        if left + 1 < i {
            quick_sort_by_descend_len(strs, left, i - 1);
        }
        quick_sort_by_descend_len(strs, i + 1, right);
    }

    pub fn compare_string(a: &String, b: &String) -> bool {
        let a_len: usize = a.len();
        let b_len: usize = b.len();
        if a_len > b_len {
            true
        } else if a_len < b_len {
            false
        } else {
            let a_bytes: &[u8] = a.as_bytes();
            let b_bytes: &[u8] = b.as_bytes();
            for i in 0..a_len {
                if a_bytes[i] < b_bytes[i] {
                    return true
                } else if a_bytes[i] > b_bytes[i] {
                    return false
                }
            }
            true
        }
    }

    pub fn quick_sort_by_func(strs: &mut Vec<String>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mut i: usize = left;
        let mut j: usize = right;
        let temp: String = strs[left].clone();
        while i < j {
            while i < j && !compare_string(&strs[j], &temp) {
                j -= 1;
            }
            strs[i] = strs[j].clone();
            while i < j && compare_string(&strs[i], &temp) {
                i += 1;
            }
            strs[j] = strs[i].clone();
        }
        strs[i] = temp;
        if left + 1 < i {
            quick_sort_by_func(strs, left, i - 1);
        }
        quick_sort_by_func(strs, i + 1, right);
    }
}

pub mod qsvec {
    pub fn quick_sort(nums: &mut Vec<Vec<i32>>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mut i: usize = left;
        let mut j: usize = right;
        let temp: Vec<i32> = nums[left].clone();
        while i < j {
            while i < j && nums[j][0] >= temp[0] {
                j -= 1;
            }
            nums[i][0] = nums[j][0];
            nums[i][1] = nums[j][1];
            while i < j && nums[i][0] <= temp[0] {
                i += 1;
            }
            nums[j][0] = nums[i][0];
            nums[j][1] = nums[i][1];
        }
        nums[i][0] = temp[0];
        nums[i][1] = temp[1];
        if left + 1 < i {
            quick_sort(nums, left, i - 1);
        }
        quick_sort(nums, i + 1, right);
    }
}

