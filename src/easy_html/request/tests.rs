// through learning and other stuff I realized a lot of these should be doc tests!

use crate::easy_html::method::Method::{self, *};
use crate::easy_html::request::Request;
use std::collections::HashMap;

// helper function to construct hashmaps from keys
fn new_map<T, U>(pairs: Vec<(T, U)>) -> HashMap<T, U>
where
    T: std::hash::Hash + std::cmp::Eq,
    U: std::hash::Hash,
{
    pairs.into_iter().collect()
}

// attempt to parse the method from a string
#[test]
fn parse_method_works() {
    let inputs = ["get", "post", "put", "delete", "POST", "GeT"];
    let expected_outputs = vec![Get, Post, Put, Delete, Post, Get];

    // map inputs to outputs
    let outputs: Vec<Method> = inputs
        .into_iter()
        .map(|input| Request::parse_method(input).unwrap())
        .collect();

    assert_eq!(outputs, expected_outputs);
}

// pass bad input to be parsed
#[test]
fn parse_method_errs_as_expected() {
    let inputs = [
        "get  ",
        "pst",
        "534378456jhkdfhks",
        "de lete",
        "P POST",
        "GeTPost",
    ];

    // map inputs to outputs
    let outputs: Vec<bool> = inputs
        .into_iter()
        .map(|input| Request::parse_method(input).is_err())
        .collect();

    // every element in outputs should be true because
    // every input should have caused an error
    let is_all_true = outputs.into_iter().all(|item| item);

    assert_eq!(is_all_true, true);
}

#[test]
fn parse_url_works() {
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
        .map(|url| Request::parse_url(url))
        .collect();

    assert_eq!(outputs, expected_outputs);
}

#[test]
fn parse_query_params_works() {
    let inputs = [
        "name=joe",
        "name=1234&age=12",
        "name=***",
        "age=123&name=456&height= =tall= ",
        "123=347",
        "=123 ",
        " 123=",
        "=123&123=",
    ];

    let expected_outputs = vec![
        new_map(vec![("name", "joe")]),
        new_map(vec![("name", "1234"), ("age", "12")]),
        new_map(vec![("name", "***")]),
        new_map(vec![
            ("age", "123"),
            ("name", "456"),
            ("height", " =tall= "),
        ]),
        new_map(vec![("123", "347")]),
        new_map(vec![("", "123 ")]),
        new_map(vec![(" 123", "")]),
        new_map(vec![("", "123"), ("123", "")]),
    ];

    let outputs: Vec<HashMap<&str, &str>> = inputs
        .into_iter()
        .map(|input| Request::parse_query_string(input))
        .collect();

    assert_eq!(outputs, expected_outputs);
}

#[test]
fn parse_query_params_returns_empty_map_on_invlaid_input() {
    let inputs = [
        "&&&&&&&&&&",
        "some invalid as heck input",
        "aadasda&asdhadkada&&&AD&&AD&A&DA&DA&",
        "1234324234",
        "123213213&ajashda",
        "&&&&&-&",
        "lfdkljsfskdf",
        "sfdsfsfdsfsfdsfsfsdfdsffsdfdsfdsfsd",
        "",
    ];

    let outputs: Vec<HashMap<_, _>> = inputs
        .into_iter()
        .map(|input| Request::parse_query_string(input))
        .collect();

    for output in outputs {
        assert!(output.is_empty());
    }
}

#[test]
fn parse_query_params_ignores_invalid_input() {
    let inputs = [
        "valid=part&invalid-part",
        "invalid-part&valid=part",
        "&invalid-part&valid=part",
        "invalid-part&valid=part&",
        "&invalid-part&valid=part&",
        "&valid=part&simed123&other=valid&",
        "infsdifns&valid=part&sfsdifosfjsdf",
    ];

    let expected_outputs = [
        new_map(vec![("valid", "part")]),
        new_map(vec![("valid", "part")]),
        new_map(vec![("valid", "part")]),
        new_map(vec![("valid", "part")]),
        new_map(vec![("valid", "part")]),
        new_map(vec![("valid", "part"), ("other", "valid")]),
        new_map(vec![("valid", "part")]),
    ];

    let outputs: Vec<HashMap<_, _>> = inputs
        .into_iter()
        .map(|input| Request::parse_query_string(input))
        .collect();

    assert_eq!(outputs, expected_outputs);
}

#[test]
fn parse_query_params_ignores_duplicate_input() {
    // this test assumes
    // that the last duplicate given will be the value
    // assigned to the hashmap
    // (which depends on the .collect method)

    let inputs = [
        "age=1&age=42",
        "age=1&name=baz&age=42",
        "name=baz&age=42&age=1",
        "name=baz&age=1&name=foo&age=123",
        "name=baz&age=1&name=foo&age=123",
        "age=1&age=42&age=79",
        "=2&=asd",
    ];

    let expected_outputs = [
        new_map(vec![("age", "42")]),
        new_map(vec![("name", "baz"), ("age", "42")]),
        new_map(vec![("name", "baz"), ("age", "1")]),
        new_map(vec![("name", "foo"), ("age", "123")]),
        new_map(vec![("name", "foo"), ("age", "123")]),
        new_map(vec![("age", "79")]),
        new_map(vec![("", "asd")]),
    ];

    let outputs: Vec<HashMap<_, _>> = inputs
        .into_iter()
        .map(|input| Request::parse_query_string(input))
        .collect();

    assert_eq!(outputs, expected_outputs);
}

#[test]
fn parse_http_ver_works() {
    let inputs = [
        "HTTP/1.1",
        "HTTP/1.2",
        "HTTP/1.3",
        "HTTP/1.4",
        "HTTP/1.5",
        "HTTP/1.6",
        "HTTP/1.7",
        "HTTP/1.8",
        "HTTP/1.9",
        "HTTP/1.10", // note that this is not a current http version and so
                     // 0 is returned instead of 10 (why would the function work with
                     // invalid http versions?!)
    ];

    let expected_outputs: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    let outputs: Vec<u8> = inputs
        .into_iter()
        .map(|input| Request::parse_http_ver(input).unwrap())
        .collect();

    assert_eq!(outputs, expected_outputs);
}

#[test]
fn parse_http_ver_errors_as_expected() {
    // three error conditions
    // 1. doesn't start w "HTTP/1."
    // 2. the last character is not an integer

    let inputs = [
        "HTTP/1.", "hTTP/1.", "HTTP/2.", "HTTP/2", "HTTP/1.a", "hTTP/1.a", "HTTP/1. ", "HTTP/.1",
    ];

    // the map will panic if an error is not returned
    let _outputs: Vec<_> = inputs
        .into_iter()
        .map(|input| Request::parse_http_ver(input).unwrap_err())
        .collect(); // need this or the iterator isnt run
}

#[test]
fn parse_head_works() {
    // fails rn because I want
    // the parse header function to
    // ignore case on keys, but it currently does not

    let input = &vec![
        "some ignored first line",
        "HeaDer1: somevaluehere",
        "HeaDer2: somevaluehere",
        "another header: 2134",
        "values: 1, 2, 3, 4, 5, asd",
        ": 2134",
        "", // this line marks the end of incoming headers
    ];

    let expected_output = new_map(vec![
        ("header1", "somevaluehere"),
        ("header2", "somevaluehere"),
        ("another header", "2134"),
        ("values", "1, 2, 3, 4, 5, asd"),
        ("", "2134"),
    ]);

    let output = Request::parse_head(input);

    assert_eq!(output, expected_output);
}

#[test]
fn parse_head_returns_empty_on_all_bad_input() {
    let input = &vec![
        "some ignored first line",
        "HeaDer1 somevaluehere",
        "HeaDer2 somevaluehere",
        "another header 2134",
        "values 1, 2, 3, 4, 5, asd",
        " 2134",
        "", // this line marks the end of incoming headers
    ];

    let output = Request::parse_head(input);

    assert!(output.is_empty());
}

#[test]
fn parse_head_ignores_bad_input() {
    let input = &vec![
        "some ignored first line",
        "header1: somevaluehere",
        "header2 somevaluehere",
        "another header: 2134",
        "values 1, 2, 3, 4, 5, asd",
        ": 2134",
        "", // this line marks the end of incoming headers
    ];

    let expected_output = new_map(vec![
        ("header1", "somevaluehere"),
        // ("header2", "somevaluehere"),
        ("another header", "2134"),
        // ("values", "1, 2, 3, 4, 5, asd"),
        ("", "2134"),
    ]);

    let output = Request::parse_head(input);

    assert_eq!(output, expected_output);
}

#[test]
fn parse_head_ignores_duplicate_input() {
    let input = &vec![
        "some ignored first line",
        "header1: somevaluehere",
        "header1: somevaluehere1",
        "header2: somevaluehere",
        "another header: 2134",
        "values: 1, 2, 3, 4, 5, asd",
        ": 2134",
        ": 2135",
        "header2: somevaluehere123",
        "", // this line marks the end of incoming headers
    ];

    let expected_output = new_map(vec![
        ("header1", "somevaluehere1"),
        ("header2", "somevaluehere123"),
        ("another header", "2134"),
        ("values", "1, 2, 3, 4, 5, asd"),
        ("", "2135"),
    ]);

    let output = Request::parse_head(input);

    assert_eq!(output, expected_output);
}
