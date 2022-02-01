use std::collections::HashMap;

use handle_errors::Error;

/// Pagination struct which is getting extract
/// from query params
#[derive(Default, Debug)]
pub struct Pagination {
    /// The index of the last item which has to be returned
    pub limit: Option<i32>,
    /// The index of the first item which has to be returned
    pub offset: i32,
}

/// Extract query parameters from the `/questions` route
/// # Example query
/// GET requests to this route can have a pagination attached so we just
/// return the questions we need
/// `/questions?offset=1&limit=10`
/// # Example usage
/// ```rust
/// let query = HashMap::new();
/// query.push("limit", "10");
/// query.push("offset", "1");
/// let p = types::pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.offset, 1);
/// assert_eq!(p.limit, 10);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    // Could be improved in the future
    if  params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            // Takes the "limit" parameter in the query and tries to convert it to a number
            limit: Some(params
                .get("limit")
                .unwrap()
                .parse()
                .map_err(Error::ParseError)?),
            // Takes the "offset" parameter in the query and tries to convert it to a number
            offset: params
                .get("offset")
                .unwrap()
                .parse()
                .map_err(Error::ParseError)?,

        });
    }

    Err(Error::MissingParameters)
}
