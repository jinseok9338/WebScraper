
#[cfg(test)]
mod tests {
    use rocket::{local::blocking::Client, http::{Status,ContentType}};

    use crate::functions;

    #[test]
    fn send_request_works() {
        let rocket = rocket::build().mount("/", routes![functions::index]);
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        println!("This is test");
        assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain)); 

    }
}