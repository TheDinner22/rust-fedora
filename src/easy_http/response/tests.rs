use super::*;

#[test]
#[ignore = "fails due to HashMap not being ordered which makes test fail when they should pass"]
fn response_into_bytes_works_as_expected() {
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

#[test]
#[ignore = "fails due to HashMap not being ordered which makes test fail when they should pass"]
fn response_into_bytes_works_on_invalid_data() {
    let headers = {
        let mut map = HashMap::new();
        map.insert(
            "hi&*&(*&(*&*&(&*(*(&*(&(*)))))))".to_owned(),
            ")(**&^^%%$$##@!there".to_owned(),
        );
        map.insert(
            "this*&)(*&&*%#!@w)".to_owned(),
            "i578439857249052837(*&*^&$$%#%$#^&^%*%&^*%&^&^%$&^%s a header".to_owned(),
        );

        map
    };

    let body: Vec<u8> = "hi there! I am the body!".bytes().collect();

    let response = Response::new(1234, headers, body);

    let expected_output: Vec<u8> =
        "HTTP/1.1 1234 No Reason Phrase\r\nhi&*&(*&(*&*&(&*(*(&*(&(*))))))): )(**&^^%%$$##@!there\r\nthis*&)(*&&*%#!@w): i578439857249052837(*&*^&$$%#%$#^&^%*%&^*%&^&^%$&^%s a header\r\n\r\nhi there! I am the body!"
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
