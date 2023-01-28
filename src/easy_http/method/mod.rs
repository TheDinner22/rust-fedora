#[derive(Debug, PartialEq)]
pub enum Method {
    Post,
    Get,
    Put,
    Delete,
}

impl Method {
    pub fn try_from<T>(value: T) -> Result<Self, String>
    where
        T: ToString,
    {
        let val = value.to_string().to_lowercase();

        match val.as_str() {
            "post" => Ok(Method::Post),
            "get" => Ok(Method::Get),
            "put" => Ok(Method::Put),
            "delete" => Ok(Method::Delete),
            _ => Err(format!("invalid http method\ngot {}", val)),
        }
    }
}
