/// Установка пользовательских заголовков и параметров в адресе URL для REST запроса
use error_chain::error_chain;
use serde::Deserialize;

use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use std::collections::HashMap;
use url::Url;

#[derive(Deserialize, Debug)]
pub struct HeadersEcho {
    pub headers: HashMap<String, String>,
}

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        UrlParse(url::ParseError);
    }
}

fn main() -> Result<()> {
    let url = Url::parse_with_params(
        "http://httpbin.org/headers",
        &[("lang", "rust"), ("browser", "servo")],
    )?;

    let client = Client::new();
    let response = client
        .get(url)
        .header(USER_AGENT, "Rust-test")
        .header(
            AUTHORIZATION,
            format!("Bearer {}", "DEadBEEfc001cAFeEDEcafBAd"),
        )
        .header("X-Powered-By", "Guybrush Threepwood")
        .send()?;

    assert_eq!(
        response.url().as_str(),
        "http://httpbin.org/headers?lang=rust&browser=servo"
    );
    let out: HeadersEcho = response.json()?;
    assert_eq!(
        out.headers["Authorization"],
        "Bearer DEadBEEfc001cAFeEDEcafBAd"
    );
    assert_eq!(out.headers["User-Agent"], "Rust-test");
    assert_eq!(out.headers["X-Powered-By"], "Guybrush Threepwood");

    println!("{:?}", out);
    Ok(())
}
