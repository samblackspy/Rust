// run cargo add chrono 

use chrono::{Utc, Local};

fn main() {
    let getlocaltime = Local::now();
    println!("local time: {}", getlocaltime);

    let getutctime = Utc::now();
    println!("current time: {}", getutctime)
}