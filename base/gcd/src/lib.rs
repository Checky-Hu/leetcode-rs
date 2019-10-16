#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod gcdi32 {
    pub fn gcd(x: i32, y: i32) -> i32 {
        let (mut n1, mut n2) = if x >= y {
            (x, y)
        } else {
            (y, x)
        };
        loop {
            if n2 == 0 {
                break;
            } else {
                let t: i32 = n2;
                n2 = n1 % n2;
                n1 = t;
            }
        }
        n1
    }
}
