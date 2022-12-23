// types which make working with html requests and responses bearable

use std::collections::HashMap;

#[derive(Debug)]
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
        let val = value.to_string().trim().to_lowercase();

        match val.as_str() {
            "post" => Ok(Method::Post),
            "get" => Ok(Method::Get),
            "put" => Ok(Method::Put),
            "delete" => Ok(Method::Delete),
            _ => Err(String::from("invald http method"))
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

    fn parse_url(url_string: &str) -> Result<(&str, String), String> {
        let mut url_iter = url_string.split("?");
        
        let path = match url_iter.next() {
            Some(string) => if string.is_empty() { "/" } else { string },
            None => return Err("invalid http request".to_string()),
        };

        let raw_query_params: String = url_iter.collect();

        Ok((path, raw_query_params))
    }

    fn parse_query_string(query_params: String) -> Option<HashMap<String, String>> {
        if query_params.is_empty() { return None;}

        let query_map: HashMap<String, String> = query_params
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
            let sub_version: u8 = match sub_ver_char.try_into() {
                Ok(version) => version,
                Err(_) => return Err("invalid http request".to_string()),
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

        // if the first line is empty, the request is bad!
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
            Some(string) => Request::parse_url(string)?,
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
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    // todo test all parsing funcitons
}