use std::collections::BTreeSet;

struct SeatManager {
    set_: BTreeSet<i32>,
}

impl SeatManager {
    fn new(n: i32) -> Self {
        let mut set: BTreeSet<i32> = BTreeSet::new();
        for i in 1..=n {
            set.insert(i);
        }
        SeatManager { set_: set }
    }

    fn reserve(&mut self) -> i32 {
        let seat_number: i32 = *self.set_.iter().next().unwrap_or(&0);
        self.set_.remove(&seat_number);
        seat_number
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.set_.insert(seat_number);
    }
}

fn main() {
    let mut seat_manager: SeatManager = SeatManager::new(5);
    println!("reserve: {}", seat_manager.reserve());
    println!("reserve: {}", seat_manager.reserve());
    seat_manager.unreserve(2);
    println!("reserve: {}", seat_manager.reserve());
    println!("reserve: {}", seat_manager.reserve());
    println!("reserve: {}", seat_manager.reserve());
    println!("reserve: {}", seat_manager.reserve());
    seat_manager.unreserve(5);
}
