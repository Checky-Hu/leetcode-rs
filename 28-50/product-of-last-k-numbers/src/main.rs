use std::cmp::Ordering;

struct ProductOfNumbers {
    products_: Vec<i32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        ProductOfNumbers {
            products_: Vec::new(),
        }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.products_.clear();
        } else {
            let t: i32 = match self.products_.last() {
                Some(x) => *x,
                None => 1,
            };
            self.products_.push(t * num);
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let len: usize = self.products_.len();
        match len.cmp(&(k as usize)) {
            Ordering::Greater => self.products_[len - 1] / self.products_[len - 1 - k as usize],
            Ordering::Equal => self.products_[len - 1],
            Ordering::Less => 0,
        }
    }
}

fn main() {
    let mut obj: ProductOfNumbers = ProductOfNumbers::new();
    obj.add(3);
    obj.add(0);
    obj.add(2);
    obj.add(5);
    obj.add(4);
    println!("Product of 2: {}", obj.get_product(2));
    println!("Product of 3: {}", obj.get_product(3));
    println!("Product of 4: {}", obj.get_product(4));
    obj.add(8);
    println!("Product of 2: {}", obj.get_product(2));
}
