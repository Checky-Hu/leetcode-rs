use std::cmp::Ordering;

struct MyCalendar {
    v_: Vec<Vec<i32>>,
}

impl MyCalendar {
    fn new() -> Self {
        MyCalendar { v_: Vec::new() }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let len: usize = self.v_.len();
        let mut pos: usize = len;
        for (i, v) in self.v_.iter().enumerate() {
            match (*v)[0].cmp(&start) {
                Ordering::Less => {
                    if (*v)[1] > start {
                        return false;
                    }
                }
                Ordering::Equal => return false,
                Ordering::Greater => match (*v)[0].cmp(&end) {
                    Ordering::Less => return false,
                    _ => {
                        pos = i;
                        break;
                    }
                },
            }
        }
        self.v_.insert(pos, vec![start, end]);
        true
    }
}

fn main() {
    let mut obj: MyCalendar = MyCalendar::new();
    println!("book [10, 20): {}", obj.book(10, 20));
    println!("book [15, 25): {}", obj.book(15, 25));
    println!("book [20, 30): {}", obj.book(20, 30));
}
