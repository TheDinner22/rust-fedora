use super::*;

#[test]
fn into_bytes_works_as_expected() {
    let headers = {
        let mut map = HashMap::new();
        map.insert("hi".to_owned(), "there".to_owned());
        map.insert("this".to_owned(), "is a header".to_owned());

        map
    };

    let body: Vec<u8> = "hi there! I am the body!".bytes().collect();

    let response = Response::new(200, headers, body);

    let expected_output: Vec<u8> =
        "HTTP/1.1 200 OK\r\nhi: there\r\nthis: is a header\r\n\r\nhi there! I am the body!"
            .bytes()
            .collect();

    let output = response.into_bytes();

    // human readable
    assert_eq!(
        std::str::from_utf8(&expected_output),
        std::str::from_utf8(&output)
    );
    // not human readable
    assert_eq!(expected_output, output);
}
