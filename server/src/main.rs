#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod functions;

fn main() {
    rocket::ignite().mount("/", routes![functions::index]).launch();
}