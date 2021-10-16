use crate::document::Document;
use regex::Regex;
use soup::prelude::*;
use url::Url;

fn valid_scheme(url: &Url) -> bool {
    Regex::new(r"https?").unwrap().is_match(url.scheme())
}

pub fn extract_from_html(document: &Document) -> Vec<String> {
    let soup = Soup::new(document.content());
    let all_hrefs = soup.tag("a").find_all().filter_map(|tag| tag.get("href"));

    // I assume that if use downloaded something by URL, it should be correct
    let base_url = Url::parse(document.url()).unwrap();

    let absolute_urls: Vec<_> = all_hrefs
        .into_iter()
        .filter_map(|href| base_url.join(&href).ok())
        .filter(valid_scheme)
        .map(|url| String::from(url.as_str()))
        .collect();

    absolute_urls
}
