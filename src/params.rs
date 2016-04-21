use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use iron::prelude::*;
use iron::status;
use iron::Url;
use padding::PaddingRequest;

#[derive(Debug)]
pub enum ParamsError {
    MissingParams,
    MissingContent,
    MissingLength,
    BadLength,
    LengthTooLong,
}

impl fmt::Display for ParamsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParamsError {
    fn description(&self) -> &str {
        match *self {
            ParamsError::MissingParams => "params are missing",
            ParamsError::MissingContent => "str is missing",
            ParamsError::MissingLength => "missing length",
            ParamsError::BadLength => "unable to parse len",
            ParamsError::LengthTooLong => "illegal padding length",
        }
    }
}

impl From<ParamsError> for IronError {
    fn from(error: ParamsError) -> IronError {
        IronError {
            response: Response::with((status::BadRequest, error.description())),
            error: box error,
        }
    }
}

pub fn read_params(url: &Url) -> Result<PaddingRequest, ParamsError> {
    let query = url.clone().into_generic_url();
    match query.query_pairs() {
        None => Err(ParamsError::MissingParams),
        Some(params) => {
            let key_value_pairs: HashMap<_, _> = params.into_iter().collect();
            
            let length = key_value_pairs.get("len")
                .ok_or(ParamsError::MissingLength)
                .and_then(|length| length.parse().map_err(|_| ParamsError::MissingLength));
                    
            match length {
                Err(e) => Err(e),
                Ok(length) if length > 999 => Err(ParamsError::LengthTooLong),
                Ok(length) => Ok(PaddingRequest::new(
                    length,
                    key_value_pairs.get("str").ok_or(ParamsError::MissingContent).map(|content| content.to_owned())?,
                    key_value_pairs.get("ch").and_then(|ch| ch.chars().nth(0)).unwrap_or(' '),    
                )),
            }
        }
    }
}