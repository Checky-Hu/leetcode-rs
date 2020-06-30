struct Cashier {
    n_: i32,
    t_: i32,
    discount_: i32,
    values_: Vec<i32>,
}

impl Cashier {
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let mut values: Vec<i32> = vec![0; 200];
        for (i, product) in products.iter().enumerate() {
            values[*product as usize - 1] = prices[i];
        }
        Cashier {
            n_: n,
            t_: 0,
            discount_: discount,
            values_: values,
        }
    }

    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        let mut bill: f64 = 0_f64;
        for (i, v) in product.iter().enumerate() {
            bill += (self.values_[*v as usize - 1] * amount[i]) as f64;
        }
        self.t_ += 1;
        if self.t_ == self.n_ {
            self.t_ = 0;
            bill * (100 - self.discount_) as f64 / 100_f64
        } else {
            bill
        }
    }
}

fn main() {
    let mut obj: Cashier = Cashier::new(
        3,
        50,
        vec![1, 2, 3, 4, 5, 6, 7],
        vec![100, 200, 300, 400, 300, 200, 100],
    );
    println!("Get bill: {}", obj.get_bill(vec![1, 2], vec![1, 2]));
    println!("Get bill: {}", obj.get_bill(vec![3, 7], vec![10, 10]));
    println!(
        "Get bill: {}",
        obj.get_bill(vec![1, 2, 3, 4, 5, 6, 7], vec![1, 1, 1, 1, 1, 1, 1])
    );
    println!("Get bill: {}", obj.get_bill(vec![4], vec![10]));
    println!("Get bill: {}", obj.get_bill(vec![7, 3], vec![10, 10]));
    println!(
        "Get bill: {}",
        obj.get_bill(vec![7, 5, 3, 1, 6, 4, 2], vec![10, 10, 10, 9, 9, 9, 7])
    );
    println!("Get bill: {}", obj.get_bill(vec![2, 3, 5], vec![5, 3, 2]));
}
