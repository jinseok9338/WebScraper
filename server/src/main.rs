#[macro_use] extern crate rocket;





mod functions;
#[allow(non_snake_case)]
mod CORS;
mod test;

#[launch] 
fn rocket() -> _ {
    rocket::build()
        .attach(CORS::CORS)
        .mount("/", routes![functions::index,functions::get_the_price_of_ticker])
}

