use reqwest::blocking::Response;

fn valid_content_type(response: &Response) -> bool {
    response.headers()
        .get("content-type")
        .map(|value| value.to_str().ok())
        .flatten()
        .map(|value| value.starts_with("text/html"))
        .is_some()
}

pub fn download(url: &str) -> Option<String> {
    reqwest::blocking::get(url)
        .ok()
        .filter(valid_content_type)
        .map(|response| response.text().ok())
        .flatten()
}