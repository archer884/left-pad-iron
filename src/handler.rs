use iron::prelude::*;
use iron::status;
use padding;
use params;
use rustc_serialize::json;

pub fn help(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        "curl <server>/api/v1/pad?str=John%20Smith&len=30&char=%20",
    )))
}

pub fn pad(request: &mut Request) ->  IronResult<Response> {
    left(request)
}

pub fn left(request: &mut Request) ->  IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        json::encode(&padding::left(params::read_params(&request.url)?)).unwrap(),
    )))
}

pub fn right(request: &mut Request) ->  IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        json::encode(&padding::right(params::read_params(&request.url)?)).unwrap(),
    )))
}