use futures::SinkExt;
use reqwest;
use reqwest::{header, StatusCode};
use reqwest::cookie::Cookie;

static BASE_URL: &'static str = "https://judge.science.unitn.it";

#[tokio::main]
async fn get__xsrf() -> Result<String, reqwest::Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert("User-Agent", header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36"));


    let client = reqwest::Client::builder().cookie_store(true).default_headers(headers).build()?;

    let response = client.get(BASE_URL).send().await?;
    return match response.status() {
        StatusCode::OK => {
            let cookie: Cookie = response.cookies().last().unwrap();
            let cookie_value: &'static str = cookie.value();
            // println!("{}", cookie_value);
            let form = reqwest::multipart::Form::new()
                .text("username", "stud016")
                .text("_xsrf", cookie_value)
                .text("password", "sfregipunzone86");
            let second_attempt = client.post(format!("{BASE_URL}/login")).multipart(form).send().await;
            println!("The second outcome was: {}", second_attempt.unwrap().status());
            Ok("brbrbbr".to_string())
        }
        _ => { Ok("no".to_string()) }
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        get__xsrf();
    }
}