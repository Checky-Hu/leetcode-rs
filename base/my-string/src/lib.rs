#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod stringop {
    pub fn add_string(s1: &String, s2: &String) -> String {
        let (s1_bytes, l1, s2_bytes, l2) = if s1.len() >= s2.len() {
            (s2.as_bytes(), s2.len(), s1.as_bytes(), s1.len())
        } else {
            (s1.as_bytes(), s1.len(), s2.as_bytes(), s2.len())
        };
        let mut i: i32 = l1 as i32 - 1;
        let mut j: i32 = l2 as i32 - 1;
        let mut carry: u8 = 0;
        let mut result: Vec<u8> = Vec::with_capacity(l2 + 1);
        while i >= 0 {
            let t: u8 = carry + s1_bytes[i as usize] + s2_bytes[j as usize] - 96;
            if t >= 10 {
                carry = 1;
                result.push(t + 38);
            } else {
                carry = 0;
                result.push(t + 48);
            }
            i -= 1;
            j -= 1;
        }
        while j >= 0 {
            let t: u8 = carry + s2_bytes[j as usize] - 48;
            if t >= 10 {
                carry = 1;
                result.push(t + 38);
            } else {
                carry = 0;
                result.push(t + 48);
            }
            j -= 1;
        }
        if carry == 1 {
            result.push(49);
        }
        result.reverse();
        String::from_utf8(result).unwrap()
    }
}
