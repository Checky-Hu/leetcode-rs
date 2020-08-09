use std::env;
use std::str::Bytes;

struct Solution {}

impl Solution {
    pub fn reformat_date(date: String) -> String {
        let vec: Vec<&str> = date.split_whitespace().collect();
        let mut result: String = String::new();
        result.push_str(vec[2]);
        result.push('-');
        result.push_str(match vec.get(1) {
            Some(x) => match *x {
                "Jan" => "01",
                "Feb" => "02",
                "Mar" => "03",
                "Apr" => "04",
                "May" => "05",
                "Jun" => "06",
                "Jul" => "07",
                "Aug" => "08",
                "Sep" => "09",
                "Oct" => "10",
                "Nov" => "11",
                _ => "12",
            },
            None => "01",
        });
        result.push('-');
        let mut bytes: Bytes = vec[0].bytes();
        if bytes.len() == 3 {
            result.push('0');
            result.push(bytes.next().unwrap() as char);
        } else {
            result.push(bytes.next().unwrap() as char);
            result.push(bytes.next().unwrap() as char);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let date: String = arg;
            println!("Reformat date: {}", Solution::reformat_date(date));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
