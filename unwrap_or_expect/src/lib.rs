// unwrap_or_expect/src/lib.rs
#[derive(Debug)]
pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match (server, security_level) {
        (Ok(url), Security::Warning) => format!("{}", url),
        (Err(e), Security::Warning) => format!("WARNING: check the server"),
        (Err(e), Security::NotFound) => format!("Not found: {}", e),
        (Err(""), Security::Message) => panic!("ERROR: program stops"),
        (Err(e), Security::Unknown) => panic!("called `Result::unwrap()` on an `Err` value: \"ERROR CRITICAL\""),
        (Ok(e), Security::UnexpectedUrl) => panic!("{}",e),
        (Ok(url), _) => url.to_string(),
        (Err(e), _) => e.to_string(),
    }
}
