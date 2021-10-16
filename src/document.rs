use reqwest::blocking::Response;

pub struct Document<'a> {
    url: &'a str,
    content: String
}

fn valid_content_type(response: &Response) -> bool {
    response.headers()
        .get("content-type")
        .map(|value| value.to_str().ok())
        .flatten()
        .map(|value| value.starts_with("text/html"))
        .is_some()
}

impl<'a> Document<'a> {
    pub fn download(url: &'a str) -> Option<Self> {
        reqwest::blocking::get(url)
            .ok()
            .filter(valid_content_type)
            .map(|response| response.text().ok())
            .flatten()
            .map(|content| Self {
                url,
                content
            })
    }

    pub fn url(&self) -> &'a str {
        self.url
    }

    pub fn content(&self) -> &String {
        &self.content
    }
}


