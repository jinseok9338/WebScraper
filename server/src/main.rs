#[macro_use] extern crate rocket;


use tokio::task::spawn_blocking;
use yahoo_finance_api as yahoo;
use chrono::{Utc,TimeZone};
use rocket::response::content;




#[get("/")]
async fn index() -> content::Json<String> {
    let provider = yahoo::YahooConnector::new();
    let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    let end = Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999);
    // returns historic quotes with daily interval
    let resp = spawn_blocking(move || {provider.get_quote_history("AAPL", start, end)}).await;
    let quotes = resp.unwrap().unwrap();
    let formatted_quotes = format!("{:?}", quotes);
    println!("Apple's quotes in January: {:?}", quotes);
    content::Json(formatted_quotes)
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
} 