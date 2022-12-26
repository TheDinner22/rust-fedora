// types which make working with html requests and responses bearable

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Method {
    Post,
    Get,
    Put,
    Delete
}

impl Method {
    fn try_from<T>(value: T) -> Result<Self, String>
    where
        T: ToString
    {
        let val = value.to_string().to_lowercase();

        match val.as_str() {
            "post" => Ok(Method::Post),
            "get" => Ok(Method::Get),
            "put" => Ok(Method::Put),
            "delete" => Ok(Method::Delete),
            _ => Err(String::from("invalid http request"))
        }
    }
}

pub struct Response {
    status_code: u16, // todo is u32 the right size??
    headers: String, // todo is it type String???
    body: String, // todo is it type String???
}

impl From<u16> for Response {
    fn from(status_code: u16) -> Self {
        Response { status_code, headers: "".to_string(), body: "".to_string() }
    }
}

#[derive(Debug)]
pub struct Request {
    body: Option<String>,
    headers: HashMap<String, String>,
    query_string_object: Option<HashMap<String, String>>,
    path: String, // todo is it type String???
    method: Method,
    http_ver: u8
}

impl Request {
    fn parse_method(method_str: &str) -> Result<Method, String> {
        Method::try_from(method_str)
    }

    fn parse_url(url_string: &str) -> (&str, &str) {
        let (mut raw_path, raw_params) = url_string
            .split_once("?")
            .unwrap_or( (url_string, "") );

        if raw_path.is_empty() {
            raw_path = "/";
        }

        (raw_path, raw_params)
    }

    /// parse a string containing query params
    /// 
    /// # Panics
    /// 
    /// This function should never panic
    /// 
    /// # Behavior
    /// 
    /// This function shoud
    /// 
    /// 1. Only return `Some<HashMap>` if there is at least one valid query parameter
    /// 2. return `None` if there were no valid query parameters
    /// 3. ignore invalid query parameters
    /// 4 duplicate fields will be ignored! (only one will be returned in the hashmap)
    /// 
    /// That is, this function will never return an empty hashmap.
    /// 
    /// Additionally, in a query string such as
    /// 
    /// >
    /// > "/path/page?&jsdhfsdfkj&&JHKJH&&&Jjgdfhk&name=joe"
    /// >
    /// 
    /// the invalid parts of the string will be ignored.
    /// 
    fn parse_query_string(query_params: &str) -> Option<HashMap<String, String>> {
        if query_params.is_empty() { return None;}

        let query_map: HashMap<_, _> = query_params
            .split("&")
            .filter_map(|pair| pair.split_once("="))
            .map(|(key, value)| (key.to_owned(), value.to_owned()))
            .collect();

        Some(query_map) // todo could this accidentally be empty?
    }

    fn parse_http_ver(http_ver_str: &str) -> Result<u8, String> {
        const EXPECT: &str = "HTTP/1.";

        if http_ver_str.starts_with(EXPECT) {
            // get the last character from the http request string
            let sub_ver_char = match http_ver_str.chars().last() {
                Some(char) => char,
                None => return Err("invalid http request".to_string()),
            };


            // try to parse the char into a u8
            let sub_version: u8 = match sub_ver_char.to_digit(10) {
                Some(version) => version as u8,
                None => return Err("invalid http request".to_string()),
            };

            Ok(sub_version)
        }
        else {
            Err("invalid http request".to_string())
        }
    }

    fn parse_head(request_as_lines: &Vec<&str>) -> Result<HashMap<String, String>, String> {
        let mut lines_iter = request_as_lines.iter();
        lines_iter.next(); // ignore first item

        let header_map = lines_iter
            .take_while(|line| !line.is_empty())
            .filter_map(|line| line.split_once(":"))
            .map(|(s1, s2)| (s1.trim().to_owned(), s2.trim().to_owned()))
            .collect();

        Ok(header_map)
    }
}

impl TryFrom<Vec<u8>> for Request {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let http_string = match String::from_utf8(value) {
            Ok(request) => request,
            Err(e) => return Err(e.to_string()),
        };

        let lines: Vec<&str> = http_string.split("\r\n").collect();
        let first_line = *lines.first().unwrap_or(&"");

        // if the first line is empty, the request is bad! (todo refactor me!!)
        if first_line.is_empty() { return Err("invalid http request".to_string()) }

        let mut first_line_words = first_line.split_whitespace();

        // parse method
        let method = match first_line_words.next() {
            Some(string) => Request::parse_method(string)?,
            None => return Err("invalid http request".to_string()),
        };
        
        // parse url to get the path and the query parameters (if any)
        // todo this assumes urls cannot contain "?" character (it should only be used for query string stuff)
        let (raw_path, raw_query_string) = match first_line_words.next() {
            Some(string) => Request::parse_url(string),
            None => return Err("invalid http request".to_string()),
        };

        // further parse the query params into an Option<hashmap>
        let query_params = Request::parse_query_string(raw_query_string); 

        // parse http ver as x where version is 1.x
        let http_sub_ver = match first_line_words.next() {
            Some(string) => Request::parse_http_ver(string)?,
            None => return Err("invalid http request".to_string()),
        };

        // parse headers
        let headers = Request::parse_head(&lines)?;
        
        
        // parse body
        let body_str = *lines.last().unwrap_or(&"");

        let body;
        if body_str.is_empty() {
            body = None;
        }
        else {
            body = Some(body_str.to_string());
        }

        Ok(Request { body, headers, query_string_object: query_params, path: raw_path.to_string(), method, http_ver: http_sub_ver })
    }
}

#[cfg(test)]
mod tests {
    use super::{Request, Method::{*, self}};
    use std::collections::HashMap;

    // helper function to construct hashmaps from keys
    fn new_map<T, U>(pairs: Vec<(T, U)>) -> HashMap<T, U>
    where
        T: std::hash::Hash + std::cmp::Eq,
        U: std::hash::Hash
    {
        pairs.into_iter().collect()
    }

    // attempt to parse the method from a string
    #[test]
    fn parse_method_works(){
        let inputs = ["get", "post", "put", "delete", "POST", "GeT"];
        let expected_outputs = vec![Get, Post, Put, Delete, Post, Get];

        // map inputs to outputs
        let outputs: Vec<Method> = inputs
            .into_iter()
            .map( |input| Request::parse_method(input).unwrap() )
            .collect();

        assert_eq!(outputs, expected_outputs);
    }

    // pass bad input to be parsed
    #[test]
    fn parse_method_errs_as_expected(){
        let inputs = ["get  ", "pst", "534378456jhkdfhks", "de lete", "P POST", "GeTPost"];
        
        let expected_outputs = vec![
            String::from("invalid http request"),
            String::from("invalid http request"),
            String::from("invalid http request"),
            String::from("invalid http request"),
            String::from("invalid http request"),
            String::from("invalid http request"),
        ];
        
        // map inputs to outputs
        let outputs: Vec<String> = inputs
            .into_iter()
            .map( |input| Request::parse_method(input).unwrap_err() )
            .collect();

        assert_eq!(outputs, expected_outputs);
    }

    #[test]
    fn parse_url_works(){
        let inputs = vec![
            "",
            "/",
            "/?name=joe",
            "?name=joe",
            "/path/poop/////",
            "/soimetdfsjiofdfsfsfsfsfsdfggghgkjd??????????",
        ];

        let expected_outputs = [
            ("/", ""),
            ("/", ""),
            ("/", "name=joe"),
            ("/", "name=joe"),
            ("/path/poop/////", ""),
            ("/soimetdfsjiofdfsfsfsfsfsdfggghgkjd", "?????????"),
        ];

        let outputs: Vec<(&str, &str)> = inputs
            .into_iter()
            .map( |url| Request::parse_url(url) )
            .collect();

        assert_eq!(outputs, expected_outputs);
    }

    #[test]
    fn parse_query_params_works(){
        let inputs = [
            "name=joe",
            "name=1234&age=12",
            "name=***",
            "age=123&name=456&height= =tall= ",
            "123=347",
            "=123 ",
            " 123=",
            "=123&123="
        ];

        let expected_outputs = vec![
            new_map(vec![("name", "joe")]),
            new_map(vec![("name", "1234"), ("age", "12")]),
            new_map(vec![("name", "***")]),
            new_map(vec![("age", "123"), ("name", "456"), ("height", " =tall= ")]),
            new_map(vec![("123", "347")]),
            new_map(vec![("", "123 ")]),
            new_map(vec![(" 123", "")]),
            new_map(vec![("", "123"), ("123", "")]),
        ];



        let outputs: Vec<HashMap<String, String>> = inputs
            .into_iter()
            .map(|input| Request::parse_query_string(input).unwrap() )
            .collect();

        assert_eq!(outputs, expected_outputs);
    }

    // todo work on tests for other parsing functions
}