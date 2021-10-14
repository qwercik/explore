use soup::prelude::*;
use url::Url;
use regex::Regex;

fn valid_scheme(url: &Url) -> bool {
    Regex::new(r"https?")
        .unwrap()
        .is_match(url.scheme())
}

pub fn extract_from_html(html_file_content: &str) -> Vec<Url> {
    let soup = Soup::new(html_file_content);
    
    soup.tag("a")
        .find_all()
        .map(|tag| tag.get("href"))
        .filter(Option::is_some)
        .map(Option::unwrap)
        .map(|s| Url::parse(&s))
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .filter(valid_scheme)
        .collect()
}
