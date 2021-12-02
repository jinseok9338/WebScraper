use std::io::Cursor;
use rocket::{Response, http::{Status, ContentType, RawStr}};
use tokio::task::spawn_blocking;
use yahoo_finance_api as yahoo;
use chrono::{Utc,TimeZone};
use std::time::{Duration, UNIX_EPOCH};
use chrono::prelude::*;



#[get("/")]
pub async fn index() -> Result<String, String>  {
    let provider = yahoo::YahooConnector::new();
    let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    let end = Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999);
    // returns historic quotes with daily interval
    let resp = spawn_blocking(move || {provider.get_quote_history("AAPL", start, end)}).await;
    let quotes = resp.unwrap();
    match quotes{
    Ok(v) => {
        // let response = Response::build() 
        // .status(Status::Accepted)
        // .header(ContentType::Plain)
        // .raw_header("X-Teapot-Make", "Rocket")
        // .raw_header("X-Teapot-Model", "Utopia")
        // .raw_header_adjoin("X-Teapot-Model", "Series 1")
        // .sized_body(128,Cursor::new(format!("{:?}", v)))
        // .finalize();
        //  println!("Apple's quotes in January: {:?}", v); 

        return Ok(format!("{:?}", v))
    },
    Err(_e) => Err((&"something Went Wrong").to_string()) 
}
}

#[get("/<ticker>")]
pub async fn get_the_price_of_ticker(ticker:String) -> Result<String,String>{
    let provider = yahoo::YahooConnector::new();
    // get the latest quotes in 1 minute intervals
    let response = spawn_blocking(move ||{provider.get_latest_quotes(&ticker, "1m")}).await;
    // extract just the latest valid quote summery
    // including timestamp,open,close,high,low,volume
    let new_response = response.unwrap();
    // let quote = new_response.unwrap().last_quote().unwrap();
    // let time: DateTime<Utc> =DateTime::from(UNIX_EPOCH + Duration::from_secs(quote.timestamp));
    // println!("At {} quote price of Apple was {}", time.to_rfc3339(), quote.close);
    
    match new_response {
        Ok(v)=> {
            println!("{}", format!("{:?}", &v));
            Ok(format!("{:?}", v))},
        Err(e) =>Err((&"something Went Wrong").to_string())
    }

}
