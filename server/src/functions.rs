use rocket::{http::Status};
use rocket::serde::{Serialize, json};
use tokio::task::spawn_blocking;
use yahoo::{YResponse, QuoteBlock};
use yahoo_finance_api as yahoo;
use chrono::{Utc,TimeZone};
use serde::{Deserialize};
use rocket::serde::json::Json;

 



#[derive(Deserialize,Debug,Serialize)]
pub struct ResponseApi {
  status: u16,

}

#[derive(Deserialize,Debug,Serialize)]
pub struct ErrorResponse {
status: u16,
}





#[get("/")]
pub async fn index() -> Result<Json<ResponseApi> , Json<ErrorResponse> >  {
    let provider = yahoo::YahooConnector::new();
    let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    let end = Utc.ymd(2020, 1, 2).and_hms_milli(23, 59, 59, 999);
    // returns historic quotes with daily interval
    let resp = spawn_blocking(move || {provider.get_quote_history("AAPL", start, end)}).await;
    let quotes = resp.unwrap();
    match quotes{
    Ok(v) => {

      
       
        let indicators = &v.chart.result[0].indicators;
        let string_indicator = format!("{:?}", indicators);
        let split_string_1 = string_indicator.replace("QuoteBlock { quote: [QuoteList ","");
        let split_string_2:Vec<&str> = split_string_1.split("], adjclose").collect();
        let split_string_3 = split_string_2[0];

        println!("{}",&split_string_3);
        let json_split_string_2: serde_json::Value =
        serde_json::from_str(&split_string_3).expect("JSON was not well-formatted");

        println!("{:?}",json_split_string_2);

        return Ok(
            Json(ResponseApi{
                status: Status::Ok.code,
           
            })     
        )},
    Err(_e) => Err(    
        Json(ErrorResponse{
        status: Status::NotFound.code,
        
    }) ) 
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
        Err(_e) =>Err((&"something Went Wrong").to_string())
    }

}
