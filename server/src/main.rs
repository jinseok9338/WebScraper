#[macro_use] extern crate rocket;



use tokio::task::spawn_blocking;
use yahoo_finance_api as yahoo;
use chrono::{Utc,TimeZone};

use rocket::{get, routes};
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;

#[derive(Debug, Responder)]
enum Error {
    #[response(status = 400)]
    BadRequest(Template),
    #[response(status = 404)]
    NotFound(Template),
}
 
#[get("/")]
async fn index() -> Result<Redirect, Error> {
    let provider = yahoo::YahooConnector::new();
    let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    let end = Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999);
    // returns historic quotes with daily interval
    let resp = spawn_blocking(move || {provider.get_quote_history("AAPL", start, end)}).await;
    let quotes = resp.unwrap().unwrap();
    let formatted_quotes = format!("{:?}", quotes);
    let ok = Status::Ok;
    println!("Apple's quotes in January: {:?}", quotes);
    // content::Json(formatted_quotes)
    println!("{:?}", ok.code);
    ok
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