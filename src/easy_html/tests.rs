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
    
    // map inputs to outputs
    let outputs: Vec<bool> = inputs
        .into_iter()
        .map( |input| Request::parse_method(input).is_err() )
        .collect();

    // every element in outputs should be true because
    // every input should have caused an error
    let is_all_true = outputs
        .into_iter()
        .all(|item| item);

    assert_eq!(is_all_true, true);
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

    let outputs: Vec<HashMap<&str, &str>> = inputs
        .into_iter()
        .map(|input| Request::parse_query_string(input).unwrap() )
        .collect();

    assert_eq!(outputs, expected_outputs);
}

// todo work on tests for other parsing functions