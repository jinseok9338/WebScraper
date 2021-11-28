#[macro_use] extern crate rocket;

use yahoo_finance_api as yahoo;
use chrono::{Utc,TimeZone};

#[get("/")]
fn index() {
    let provider = yahoo::YahooConnector::new();
    let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    let end = Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999);
    // returns historic quotes with daily interval
    let resp =provider.get_quote_history("AAPL", start, end).unwrap();
    let quotes = resp.quotes().unwrap();
    println!("Apple's quotes in January: {:?}", quotes);
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}