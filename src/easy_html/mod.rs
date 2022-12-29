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
            _ => Err(format!("invalid http method\ngot {}", val))
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
pub struct Request<'req> {
    body: Option<&'req str>,
    headers: Option<HashMap<&'req str, &'req str>>,
    query_string_object: Option<HashMap<&'req str, &'req str>>,
    path: &'req str, // todo is it type String???
    method: Method,
    http_ver: u8
}

impl<'req> Request<'req> {
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
    fn parse_query_string(query_params: &str) -> Option<HashMap<&str, &str>> {
        if query_params.is_empty() { return None;}

        let query_map: HashMap<_, _> = query_params
            .split("&")
            .filter_map(|pair| pair.split_once("="))
            .collect();

        Some(query_map) // todo could this accidentally be empty?
    }

    fn parse_http_ver(http_ver_str: &str) -> Result<u8, String> {
        const EXPECT: &str = "HTTP/1.";

        if !http_ver_str.starts_with(EXPECT) {
            return Err(format!("invalid http version\ngot {}", http_ver_str))
        }

        // get the last character from the http request string
        let sub_ver_char = match http_ver_str.chars().last() {
            Some(char) => char,
            None => return Err(format!("invalid http version\nhttp version was empty")),
        };

        // try to parse the char into a u8
        let sub_version: u8 = match sub_ver_char.to_digit(10) {
            Some(version) => version as u8,
            None => return Err(format!("invalid http version\nunable to parse")),
        };

        Ok(sub_version)
    }

    // todo are headers always given as key:value? what about commas??
    // todo bug with spaces
    fn parse_head(request_as_lines: &Vec<&'req str>) -> Option<HashMap<&'req str, &'req str>> {
        let mut lines_iter = request_as_lines.iter();
        lines_iter.next(); // ignore first item

        let header_map: HashMap<_, _> = lines_iter
            .take_while(|line| !line.is_empty())
            .filter_map(|line| line.split_once(":"))
            .collect();

        if header_map.is_empty() {
            None
        }
        else {
            Some(header_map)
        }
    }
}

impl<'req> TryFrom<&'req Vec<u8>> for Request<'req> {
    type Error = String;

    fn try_from(value: &'req Vec<u8>) -> Result<Self, Self::Error> {
        let http_string = match std::str::from_utf8(value) {
            Ok(request) => request,
            Err(e) => return Err(e.to_string()),
        };

        let lines: Vec<&str> = http_string.split("\r\n").collect();
        let first_line = *lines.first().unwrap_or(&"");

        // if the first line is empty, the request is bad! (todo refactor me!!)
        if first_line.is_empty() { return Err(format!("invalid http request\nfirst line was empty!\nheres the request:\n\n{:#?}", lines)) }

        let first_line_words: Vec<&str> = first_line
            .split_whitespace()
            .collect();

        if first_line_words.len() != 3 {
            return Err("invalid http request\nFirst line was not formatted correctly".to_string())
        }

        // parse method
        let method = Request::parse_method(first_line_words[0])?;
        
        // parse url to get the path and the query parameters (if any)
        // todo this assumes urls cannot contain "?" character (it should only be used for query string stuff)
        let (raw_path, raw_query_string) = Request::parse_url(first_line_words[1]);

        // further parse the query params into an Option<hashmap>
        let query_params = Request::parse_query_string(raw_query_string); 

        // parse http ver as x where version is 1.x
        let http_sub_ver = Request::parse_http_ver(first_line_words[2])?;

        // parse headers
        // todo why does this take the entire request if it only needs headers??
        let headers = Request::parse_head(&lines);
        
        // parse body
        let body_str = *lines.last().unwrap_or(&"");

        let body;
        if body_str.is_empty() {
            body = None;
        }
        else {
            body = Some(body_str);
        }

        Ok(Request { body, headers, query_string_object: query_params, path: raw_path, method, http_ver: http_sub_ver })
    }
}
