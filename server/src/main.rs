#[macro_use] extern crate rocket;

use rocket::http::{ContentType, Status};
use tokio::task::spawn_blocking;
use yahoo::YResponse;
use yahoo_finance_api as yahoo;
use chrono::{Utc,TimeZone};
use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
struct APIResponse { 
    res: YResponse
 }


#[get("/")]
async fn index() -> Json<APIResponse> {
    let provider = yahoo::YahooConnector::new();
    let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    let end = Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999);
    // returns historic quotes with daily interval
    let resp = spawn_blocking(move || {provider.get_quote_history("AAPL", start, end)}).await;
    let quotes = resp.unwrap().unwrap();
    println!("Apple's quotes in January: {:?}", quotes);
    Json(APIResponse{ res:quotes});
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
} 