use std::collections::HashMap;

use crate::error::Error;

#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params.get("start").unwrap().parse::<usize>().map_err(Error::ParseError)?,
            end: params.get("end").unwrap().parse::<usize>().map_err(Error::ParseError)?,
        })
    }

    Err(Error::MissingParameters)
}
