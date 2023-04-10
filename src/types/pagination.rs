use handle_errors::Error;
use std::collections::HashMap;


/// Pagination struct that is getting extracted
/// from the query parameters
#[derive(Debug, Default)]
pub struct Pagination {
    /// The index of thefirst item to be returned
    pub limit: Option<usize>,
    /// The index of the last item to be returned
    pub offset: usize,
}

/// Extract query parameters from the request
/// # Example query
/// GET requests to this route can have a pagination attached so we just
/// return the questions we need
/// `/questions?start=0&end=10`
/// # Example usage
/// ```rust
/// let mut query = HashMap::new();
/// query.insert("limit".to_string(), "1".to_string());
/// query.insert("offset".to_string(), "10".to_string());
/// let p = pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.limit, Some(1));
/// assert_eq!(p.offset, 10);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    // Could be improved in the future
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(params
                .get("limit")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?),
            offset: params
                .get("offset")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::MissingParameters)
}
