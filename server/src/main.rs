#[macro_use] extern crate rocket;


use rocket_cors::{AllowedHeaders, AllowedOrigins};
use tokio::task::spawn_blocking;
use yahoo_finance_api as yahoo;
use chrono::{Utc,TimeZone};
use rocket::{http::Method, response::content};
use std::error::Error;
use rocket::{get, routes};



 
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



#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let allowed_origins = AllowedOrigins::All;

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::build()
        .mount("/", routes![index])
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}