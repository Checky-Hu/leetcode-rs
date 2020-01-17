use std::collections::HashSet;

struct MyCalendarTwo {
    src_: HashSet<Vec<i32>>,
    dst_: HashSet<Vec<i32>>,
}

impl MyCalendarTwo {
    fn new() -> Self {
        MyCalendarTwo {
            src_: HashSet::new(),
            dst_: HashSet::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for v in self.dst_.iter() {
            if start >= (*v)[1] || end <= (*v)[0] {
                continue;
            } else {
                return false;
            }
        }
        for v in self.src_.iter() {
            if start >= (*v)[1] || end <= (*v)[0] {
                continue;
            } else {
                let s: i32 = if (*v)[0] < start { start } else { (*v)[0] };
                let e: i32 = if (*v)[1] < end { (*v)[1] } else { end };
                self.dst_.insert(vec![s, e]);
            }
        }
        self.src_.insert(vec![start, end]);
        true
    }
}

fn main() {
    let mut obj: MyCalendarTwo = MyCalendarTwo::new();
    println!("book [10, 20): {}", obj.book(10, 20));
    println!("book [50, 60): {}", obj.book(50, 60));
    println!("book [10, 40): {}", obj.book(10, 40));
    println!("book [5, 15): {}", obj.book(5, 15));
    println!("book [5, 10): {}", obj.book(5, 10));
    println!("book [25, 55): {}", obj.book(25, 55));
}
