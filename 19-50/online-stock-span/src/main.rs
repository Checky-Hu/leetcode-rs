struct StockSpanner {
    v_: Vec<(i32, i32)>,
}

impl StockSpanner {
    fn new() -> Self {
        StockSpanner { v_: Vec::new() }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut result: i32 = 1;
        while let Some(x) = self.v_.last() {
            if x.0 > price {
                break;
            } else {
                result += x.1;
                self.v_.pop();
            }
        }
        self.v_.push((price, result));
        result
    }
}

fn main() {
    let mut obj: StockSpanner = StockSpanner::new();
    println!("Next 100: {}", obj.next(100));
    println!("Next 80: {}", obj.next(80));
    println!("Next 60: {}", obj.next(60));
    println!("Next 70: {}", obj.next(70));
    println!("Next 60: {}", obj.next(60));
    println!("Next 75: {}", obj.next(75));
    println!("Next 85: {}", obj.next(85));
}
