use crate::easy_http::method::Method;
use crate::tcp_server::RawHttp;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Request<'req> {
    body: Option<Vec<u8>>, // todo not all payloads are not valid utf8
    headers: HashMap<&'req str, &'req str>,
    query_string_object: HashMap<&'req str, &'req str>,
    path: &'req str, // todo is it type String???
    method: Method,
    http_ver: u8,
}

impl<'req> Request<'req> {
    fn parse_method<T>(method_str: T) -> Result<Method, String>
    where
        T: ToString,
    {
        Method::try_from(method_str)
    }

    fn parse_url(url_string: &str) -> (&str, &str) {
        let (mut raw_path, raw_params) = url_string.split_once("?").unwrap_or((url_string, ""));

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
    /// 1. ignore invalid query parameters
    /// 2. duplicate fields will be ignored! (only one will be returned in the hashmap)
    ///
    /// That is, this function will never return an empty hashmap.
    ///
    /// So, in a query string such as
    ///
    /// >
    /// > "&jsdhfsdfkj&&JHKJH&&&Jjgdfhk&name=joe"
    /// >
    ///
    /// the invalid parts of the string will be ignored.
    ///
    fn parse_query_string(query_params: &str) -> HashMap<&str, &str> {
        let query_map: HashMap<_, _> = query_params
            .split("&")
            .filter_map(|pair| pair.split_once("="))
            .collect();

        query_map
    }

    fn parse_http_ver(http_ver_str: &str) -> Result<u8, String> {
        const EXPECT: &str = "HTTP/1.";

        if !http_ver_str.starts_with(EXPECT) {
            return Err(format!("invalid http version\ngot {}", http_ver_str));
        }

        // get the last character from the http request string
        let msg = "if http_ver_str starts w EXPECT, it must have a last character";
        let sub_ver_char = http_ver_str.chars().last().expect(msg);

        // try to parse the char into a u8
        let sub_version: u8 = match sub_ver_char.to_digit(10) {
            Some(version) => version as u8,
            None => return Err(format!("invalid http version\nunable to parse")),
        };

        Ok(sub_version)
    }

    // todo bug with being case-insensitive
    fn parse_head(headers_as_lines: &Vec<&'req str>) -> HashMap<&'req str, &'req str> {
        let lines_iter = headers_as_lines.iter();

        let header_map: HashMap<_, _> = lines_iter
            .filter_map(|line| line.split_once(":"))
            .map(|(key, val)| (key, val.trim_start()))
            .collect();

        header_map
    }
}

/// # parse an http request headers from vec of str
///
/// the &str is split on newline characters
/// and collected into a vector
///
/// todo docs for this
///
/// the body is ignored if any
///
impl<'req> TryFrom<Vec<&'req str>> for Request<'req> {
    type Error = String;

    // todo this function actually sees a lot of use in the module so please optimize it!
    fn try_from(value: Vec<&'req str>) -> Result<Self, Self::Error> {
        let lines = value; // rename bcuz trait

        let first_line = *lines.first().unwrap_or(&"");

        // if the first line is empty, the request is bad! (todo refactor me!!)
        if first_line.is_empty() {
            return Err(format!(
                "invalid http request\nfirst line was empty!\nheres the request:\n\n{:#?}",
                lines
            ));
        }

        let first_line_words: Vec<&str> = first_line.split_whitespace().collect();

        if first_line_words.len() != 3 {
            return Err("invalid http request\nFirst line was not formatted correctly".to_string());
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
        let raw_headers = lines[1..]
            .iter()
            .map(|s| *s)
            .take_while(|line| !line.is_empty())
            .collect();
        let headers = Request::parse_head(&raw_headers);

        Ok(Request {
            body: None,
            headers,
            query_string_object: query_params,
            path: raw_path,
            method,
            http_ver: http_sub_ver,
        })
    }
}

/// # parse http request from a str
///
/// usually we don't have a str (we have a tcp stream!) so this function is probably useless
impl<'req> TryFrom<&'req str> for Request<'req> {
    type Error = String;

    fn try_from(value: &'req str) -> Result<Self, Self::Error> {
        let lines: Vec<&str> = value.split("\r\n").collect();

        Request::try_from(lines)
    }
}

/// # from a &vec of u8
///
/// the Reqest will live as long as the vector does
impl<'req> TryFrom<&'req Vec<u8>> for Request<'req> {
    type Error = String;

    fn try_from(value: &'req Vec<u8>) -> Result<Self, Self::Error> {
        let http_string = match std::str::from_utf8(value) {
            Ok(request) => request,
            Err(e) => return Err(e.to_string()),
        };

        Request::try_from(http_string)
    }
}

impl<'req, 'stream> TryFrom<&'req RawHttp<'stream>> for Request<'req> {
    type Error = String;

    /// # convert a RawHttp instance into a Request
    ///
    /// this function parses the headers and then uses them to determine
    ///
    /// 1. is there a body?
    /// 2. how should that body be parsed
    ///
    /// # errors
    ///
    /// it will return an error if the headers are unable to be parsed
    ///
    /// all other errors have to do with determining how to parse the body and parsing the body
    /// here are the rules this function uses to parse the request body:
    /// https://greenbytes.de/tech/webdav/rfc7230.html#message.body.length
    ///
    /// #body parsing tldr:
    ///
    /// if the transfer_encoding is chunked we use that
    /// if transfer_encoding is not chunked the request is invalid
    /// if there is transfer_encoding and content length then its invalid (no request smuggling!)
    /// multiple content lengths or a content_length with an ivalid value means an error
    /// if theres no transfer_encoding but there is a content length we use that
    /// otherwise, the request has no body
    fn try_from(value: &'req RawHttp) -> Result<Self, Self::Error> {
        // convert the first line and headers into a &str
        let owned_lines = value.raw_headers();
        let lines: Vec<&str> = owned_lines.iter().map(|line| &**line).collect(); // see https://stackoverflow.com/questions/33216514/how-do-i-convert-a-vecstring-to-vecstr

        let mut request_without_body = Request::try_from(lines)?;

        // now we parse the headers to determine if there is a body and how to parse it
        let headers = &request_without_body.headers;

        let content_length = headers.get("Content-Length");
        let transfer_encoding = headers.get("Transfer-Encoding");

        let raw_body: Option<Vec<u8>> = match (content_length, transfer_encoding) {
            (None, None) => None,

            (Some(_), Some(_)) => {
                return Err(String::from(
                    "bad request: transfer encoding and content length provided",
                ));
            }

            (None, Some(encoding)) => {
                if *encoding != "Chunked" {
                    return Err(String::from(
                        "bad request: transfer encoding was not Chunked",
                    ));
                }

                // cannot yet handle this kind of request
                todo!()
            }

            (Some(raw_length), None) => {
                let length = raw_length.parse::<usize>().map_err(|e| e.to_string())?;

                let body = value.take_body_stream(length).map_err(|e| e.to_string())?;

                Some(body) //once told me
            }
        };

        // add the body we just parsed (which may still be none) to the (now poorly named request)
        request_without_body.body = raw_body;

        Ok(request_without_body)
    }
}

#[cfg(test)]
mod tests;
