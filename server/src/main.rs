#[macro_use] extern crate rocket;



use tokio::task::spawn_blocking;
use yahoo_finance_api as yahoo;
use chrono::{Utc,TimeZone};

use rocket::{get, routes, Error};
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

use rocket::http::{ContentType, Status};

use std::io::Cursor;




#[get("/")]
async fn index() -> Result<String, String>  {
    let provider = yahoo::YahooConnector::new();
    let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    let end = Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999);
    // returns historic quotes with daily interval
    let resp = spawn_blocking(move || {provider.get_quote_history("AAPL", start, end)}).await;
    let quotes = resp.unwrap();
    let formatted_quotes = format!("{:?}", quotes);
    match quotes{
    Ok(v) => {
        let response = Response::build()
        .status(Status::Accepted)
        .header(ContentType::Plain)
        .raw_header("X-Teapot-Make", "Rocket")
        .raw_header("X-Teapot-Model", "Utopia")
        .raw_header_adjoin("X-Teapot-Model", "Series 1")
        .sized_body(128,Cursor::new(format!("{:?}", v)))
        .finalize();
         println!("Apple's quotes in January: {:?}", v);

        return Ok(format!("{:?}", v))
    },
    Err(e) => Err((&"something Went Wrong").to_string())
}
    
   

  
}


pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}




#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![index])
}