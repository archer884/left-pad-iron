#![feature(box_syntax, question_mark)]

extern crate iron;
#[macro_use] extern crate router;
extern crate pad;
extern crate rustc_serialize;

mod handler;
mod padding;
mod params;

use iron::Iron;

fn main() {
    Iron::new(router! {
        get "/" => handler::help,
        get "/api/v1/pad" => handler::pad,
        get "/api/v2/left" => handler::left,
        get "/api/v2/right" => handler::right,
    }).http("localhost:1337").unwrap();
}
