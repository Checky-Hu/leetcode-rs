use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn is_leap_year(y: i32) -> bool {
        y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
    }

    fn get_days_of_m_by_y(y: i32, m: i32) -> i32 {
        match m {
            2 => {
                if Solution::is_leap_year(y) {
                    29
                } else {
                    28
                }
            }
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            _ => 30,
        }
    }

    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        let vec1: Vec<&str> = date1.split('-').collect();
        let y1: i32 = i32::from_str(vec1[0]).unwrap_or_default();
        let m1: i32 = i32::from_str(vec1[1]).unwrap_or_default();
        let d1: i32 = i32::from_str(vec1[2]).unwrap_or_default();
        let vec2: Vec<&str> = date2.split('-').collect();
        let y2: i32 = i32::from_str(vec2[0]).unwrap_or_default();
        let m2: i32 = i32::from_str(vec2[1]).unwrap_or_default();
        let d2: i32 = i32::from_str(vec2[2]).unwrap_or_default();
        let mut result: i32 = 0;
        match y1.cmp(&y2) {
            Ordering::Equal => match m1.cmp(&m2) {
                Ordering::Equal => match d1.cmp(&d2) {
                    Ordering::Equal => 0,
                    Ordering::Greater => d1 - d2,
                    Ordering::Less => d2 - d1,
                },
                Ordering::Greater => {
                    result += Solution::get_days_of_m_by_y(y2, m2) - d2 + d1;
                    for i in (m2 + 1)..m1 {
                        result += Solution::get_days_of_m_by_y(y2, i);
                    }
                    result
                }
                Ordering::Less => {
                    result += Solution::get_days_of_m_by_y(y1, m1) - d1 + d2;
                    for i in (m1 + 1)..m2 {
                        result += Solution::get_days_of_m_by_y(y1, i);
                    }
                    result
                }
            },
            Ordering::Greater => {
                result += Solution::get_days_of_m_by_y(y2, m2) - d2;
                for i in (m2 + 1)..=12 {
                    result += Solution::get_days_of_m_by_y(y2, i);
                }
                for i in (y2 + 1)..y1 {
                    result += if Solution::is_leap_year(i) { 366 } else { 365 };
                }
                for i in 1..m1 {
                    result += Solution::get_days_of_m_by_y(y1, i);
                }
                result += d1;
                result
            }
            Ordering::Less => {
                result += Solution::get_days_of_m_by_y(y1, m1) - d1;
                for i in (m1 + 1)..=12 {
                    result += Solution::get_days_of_m_by_y(y1, i);
                }
                for i in (y1 + 1)..y2 {
                    result += if Solution::is_leap_year(i) { 366 } else { 365 };
                }
                for i in 1..m2 {
                    result += Solution::get_days_of_m_by_y(y2, i);
                }
                result += d2;
                result
            }
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut date1: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            1 => date1 = arg,
            2 => {
                ret += 1;
                let date2: String = arg;
                println!(
                    "Days between dates: {}",
                    Solution::days_between_dates(date1, date2)
                );
                break;
            }
            _ => (),
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
