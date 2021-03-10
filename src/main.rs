use std::fmt;
use std::io::Write;
use std::fs::File;

/*
    To change the timeframe where you want to check the sums simply change the struct 'max_date' (being the last day) and 'date' (being the first day)
    Program exports you it's output in two forms, as a .csv with a NUMBER; AMOUNT; PERCENTAGE format and an .txt which shows index, number, amount and percentage.
*/

// Date struct which features the exact time where a tweet has been send, can also be determent to be an imperial or metric timestamp
#[derive(Copy, Clone)]
struct Date {
    minute: u32,
    hour: u32,
    day: u32,
    month: u32,
    year: u32,
    imperial: bool,
}

// Setters for vars in Date
impl Date {
    fn minute_mut(&mut self) -> &mut u32 { &mut self.minute }
    fn hour_mut(&mut self) -> &mut u32 { &mut self.hour }
    fn day_mut(&mut self) -> &mut u32 { &mut self.day }
    fn month_mut(&mut self) -> &mut u32 { &mut self.month }
    fn year_mut(&mut self) -> &mut u32 { &mut self.year }
}

// Why in the world do I need to f-ing implement an Eq-function ?!?!?!?! WHYYY (At least I can let the function ignore the imperial bool)
impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        if     self.minute  ==  other.minute
            && self.hour    ==  other.hour
            && self.day     ==  other.day
            && self.month   ==  other.month
            && self.year    ==  other.year  {
                return true;
            } else {
                return false;
            }
    }
}

// Display implementation for Date struct
impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02} {}/{}/{}", self.hour, self.minute, self.day, self.month, self.year)
    }
}

fn main() {
    let mut big_n: Vec<u32> = Vec::new();
    let max_date: Date =
        Date {  hour: 23, minute: 59,
                day: 31, month: 12, year: 2020,
                imperial: true };
    let timestamp_max_date = date_to_timestamp(max_date);

    let mut date: Date =
        Date {  hour: 0, minute: 0,
                day: 1, month: 1, year: 2020,
                imperial: true};

    let datapoints = timestamp_max_date - date_to_timestamp(date);
    let mut iteration = 1;
    while date_to_timestamp(date) != timestamp_max_date {
        big_n.push(sum_calc(date));

        println!("Itr: {} Current: {} Left: {} Max: {} SUM: {} DATE: {}", 
            iteration, 
            date_to_timestamp(date), 
            (timestamp_max_date - date_to_timestamp(date)), 
            timestamp_max_date, 
            sum_calc(date),
            date);

        iteration += 1;
        date = date_plus_plus(date);
    }

    big_n.sort();
    let mut count: Vec<u32> = Vec::new();
    let mut number: Vec<u32> = Vec::new();
    let mut iterator = 0;
    for &item in &big_n {
        println!("Sorting... {} out of {}", iterator, big_n.len());
        if !number.contains(&item) {
            number.push(item);
            count.push(1);
        } else {
            let temp = count.pop().unwrap();
            count.push(temp + 1);
        }
        iterator += 1;
    }


    let mut file_pretty = File::create("output_pretty.txt").expect("Unable to create file");
    let mut file_excel = File::create("output.csv").expect("Unable to create file");
    let number_max = number.len();
    for n in 1..number.len() {
        println!("Writing to file... {} out of {}", n, number_max);
        let count = count.pop().unwrap();
        let number = number.pop().unwrap();
        write!(file_pretty, "[{:05}] {:04}: {:05} times | Percentage: {:.8}%\n", n, number, count, count as f64 / datapoints as f64);
        write!(file_excel, "{};{};{}\n", number, count, count as f64 / datapoints as f64);
    }
}

// timestamp from 2000
fn date_to_timestamp(date: Date) -> u64 {
    let days_to_month = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366];
    if is_leap_year(date.year) {
        let days_to_month = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];
    } else {
        let days_to_month = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366];
    };

    let mut days_since_2k: u64 = 0;
    for year in 2000..date.year {
        if is_leap_year(year) {
            days_since_2k += 366;
        } else {
            days_since_2k += 365;
        };
    }
    (days_since_2k * 1440) + (days_to_month[(date.month - 1) as usize] * 1440) + ((date.day * 1440) as u64) + ((date.hour * 60) as u64) + ((date.minute) as u64)
}

// add one minute to a Date struct
fn date_plus_plus(mut date: Date) -> Date {
    let month_days= [31, 28, 31, 30 ,31, 30, 31, 31, 30, 31, 30, 31];
    if date.minute == 59 {
        *date.minute_mut() = 0;
        if date.hour == 23 {
            *date.hour_mut() = 0;
            if date.day == month_days[(date.month - 1) as usize] {
                *date.day_mut() = 1;
                if date.month == 12 {
                    *date.month_mut() = 1;
                    *date.year_mut() = date.year + 1;
                } else {
                    *date.month_mut() = date.month + 1;
                };
            } else {
                *date.day_mut() = date.day + 1;
            };
        } else {
            *date.hour_mut() = date.hour + 1;
        };
    } else {
        *date.minute_mut() = date.minute + 1;
    };
    date
}

// calculates sum that conspiracy fools use to see similarities
fn sum_calc (date: Date) -> u32 {
    if date.imperial {
        return date.day + (metric_to_imperial_hour(date.hour) * 100) + date.minute + date.month + date.year;
    } else {
        return date.day + (date.hour * 100) + date.minute + date.month + date.year;
    };
}

// return imperial hour from metric hour count
fn metric_to_imperial_hour (hour: u32) -> u32 {
    if hour <= 12 {
        if hour == 0 {
            return 12;
        } else {
            return hour;
        } 
    } else {
        return hour - 12;
    };
}

// n being calculated by amount of hours, minutes, average days in a month, months in a specific year
// If you wanna code in more analytical stuff, have fun!!!
fn n_year (year: u32) -> u32 {
    return (24. * 60. * avg_days_in_month(is_leap_year(year)) * 12.) as u32;
}

// returns average days in the months of a (non-)leap year
// Could be used for more analytical stuff
fn avg_days_in_month (leap_year: bool) -> f32 {
    if leap_year {
        return 365. / 12.;
    } else {
        return 366. / 12.;
    }
}

// returns if year is leap year
fn is_leap_year (year: u32) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                return true;
            } else {
                return false;
            };
        } else {
            return true;
        };
    } else {
        return false;
    };
}

/*  If you ever wanna use mergesort instead of timsort
/   Actual reason being that I started coding my own (terribly made) sorting
/   algorithm until I realized that lists implement their own sort function
fn mergesort(vector: Vec<u32>) -> Vec<u32> {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    let mut result: Vec<u32> = Vec::new();
    if vector.len() <= 1 {
        return vector;
    } else {
        let middle = vector.len() / 2;
        let mut iterator = 0;
        for item in vector {
            if iterator < middle { 
                left.push(item) 
            } else {
                right.push(item)
            }
        }
        left = mergesort(left);
        right = mergesort(right);
        if left.last() <= right.first() {
            left.append(&mut right);
            return result;
        }
        return merge(left, right);
    };
}

fn merge(mut left: Vec<u32>, mut right: Vec<u32>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    while left.len() > 0 && right.len() > 0 {
        if left.first() <= right.first() {
            result.push(*left.first().unwrap());
            left.drain(1..);
        }  else {
            result.push(*right.first().unwrap());
            right.drain(1..);
        }
    };
    if left.len() > 0 {
        result.append(&mut left);
    };
    if right.len() > 0 {
        result.append(&mut right);
    };
    return result;
}
*/