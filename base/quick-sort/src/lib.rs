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

use std::cmp::Ordering;

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

    pub fn compare_string(a: &str, b: &str) -> bool {
        let a_len: usize = a.len();
        let b_len: usize = b.len();
        match a_len.cmp(&b_len) {
            Ordering::Greater => true,
            Ordering::Less => false,
            Ordering::Equal => {
                let a_bytes: &[u8] = a.as_bytes();
                let b_bytes: &[u8] = b.as_bytes();
                for i in 0..a_len {
                    match a_bytes[i].cmp(&b_bytes[i]) {
                        Ordering::Less => return true,
                        Ordering::Greater => return false,
                        Ordering::Equal => (),
                    }
                }
                true
            }
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

    pub fn compare_string_in_dict(a: &str, b: &str) -> bool {
        let a_bytes: &[u8] = a.as_bytes();
        let a_len: usize = a_bytes.len();
        let b_bytes: &[u8] = b.as_bytes();
        let b_len: usize = b_bytes.len();
        let mut i: usize = 0;
        while i < a_len && i < b_len {
            match a_bytes[i].cmp(&b_bytes[i]) {
                Ordering::Less => return true,
                Ordering::Greater => return false,
                Ordering::Equal => (),
            }
            i += 1;
        }
        i == a_len
    }

    pub fn quick_sort_by_dict(strs: &mut Vec<String>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mut i: usize = left;
        let mut j: usize = right;
        let temp: String = strs[left].clone();
        while i < j {
            while i < j && !compare_string_in_dict(&strs[j], &temp) {
                j -= 1;
            }
            strs[i] = strs[j].clone();
            while i < j && compare_string_in_dict(&strs[i], &temp) {
                i += 1;
            }
            strs[j] = strs[i].clone();
        }
        strs[i] = temp;
        if left + 1 < i {
            quick_sort_by_dict(strs, left, i - 1);
        }
        quick_sort_by_dict(strs, i + 1, right);
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

pub mod qstuple {
use std::cmp::Ordering;

    pub fn compare_tuple(a: &(i32, String), b: &(i32, String)) -> bool {
        match a.0.cmp(&b.0) {
            Ordering::Greater => true,
            Ordering::Less => false,
            Ordering::Equal => {
                let a_bytes: &[u8] = a.1.as_bytes();
                let a_len: usize = a_bytes.len();
                let b_bytes: &[u8] = b.1.as_bytes();
                let b_len: usize = b_bytes.len();
                let mut i: usize = 0;
                while i < a_len && i < b_len {
                    match a_bytes[i].cmp(&b_bytes[i]) {
                        Ordering::Less => return true,
                        Ordering::Greater => return false,
                        Ordering::Equal => (),
                    }
                    i += 1;
                }
                i == a_len
            }
        }
    }

    pub fn quick_sort_by_func(tuples: &mut Vec<(i32, String)>, left: usize, right: usize) {
        if left >= right {
            return;
        }

        let mut i: usize = left;
        let mut j: usize = right;
        let temp: (i32, String) = tuples[left].clone();
        while i < j {
            while i < j && !compare_tuple(&tuples[j], &temp) {
                j -= 1;
            }
            tuples[i] = tuples[j].clone();
            while i < j && compare_tuple(&tuples[i], &temp) {
                i += 1;
            }
            tuples[j] = tuples[i].clone();
        }
        tuples[i] = temp;
        if left + 1 < i {
            quick_sort_by_func(tuples, left, i - 1);
        }
        quick_sort_by_func(tuples, i + 1, right);
    }
}
