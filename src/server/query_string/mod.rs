use std::collections::HashMap;

/// # HashMap containing query string object
/// in short, this struct wraps a &'str which represents 
/// an incoming request's query string object and parses that
/// &str into a HashMap for you.
///
/// ## examples
/// ```
/// // valid query_params are parsed into HashMap
/// let URI = "https://some_website/users.com?password=123&foo=bar";
/// let query_params = LazyQueryString::new(URI);
/// assert_eq!(2, query_params.query_params().len());
///
/// // invalid params are ignored
/// let URI = "https://some_website/users.com?these are all invalid params and will be
/// ignored&valid= params are still parsed";
/// let query_params = LazyQueryString::new(URI);
/// assert_eq!(1, query_params.query_params().len());
///
/// ```
///
/// ## TODOS
/// I would like to use std::cell::LazyCell for lazy loading; however, it is still not allowed on stable rust.
/// If I want to try to add Lazy loading in the future,
/// this page https://stackoverflow.com/questions/29401626/how-do-i-return-a-reference-to-something-inside-a-refcell-without-breaking-encap
/// shows a way I could do lazy loading with a RefCell and still return a reference (not A std::cell::Ref) 
/// (or at least a struct which could be coerced into a reference).
///
/// ## implementation
/// Callig the new function parses the given string and creates a HashMap.
/// You can the get a reference to that HashMap with the query_params method. 
/// Thats it!
///
/// repeated fields are ignored (only one is stored in the HashMap)!
/// invalid fields are also ignored
///
/// ## panic
///
/// this struct and its methods should never panic.
pub struct QueryString<'req> {
    param_map: HashMap<&'req str, &'req str>
}

impl<'req> QueryString<'req> {
    /// # parse a raw query_string into a HashMap
    pub fn new(query_string: &'req str) -> Self {
        let query_map = query_string
            .split('&')
            .filter_map(|pair| pair.split_once('='))
            .collect();

        QueryString { param_map: query_map }
    }

    /// # returns a reference to the wrapped HashMap
    pub fn query_params(&self) -> &HashMap<&str, &str> {
        &self.param_map
    }
}

