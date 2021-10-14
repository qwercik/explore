use url::Url;
use regex::Regex;
use std::collections::{VecDeque, HashMap};
use crate::cli::Options;
use crate::downloader::download;
use crate::url_extractor::extract_from_html;

fn is_url_domain_ok(domain_regex: &Regex, url: &Url) -> bool {
    url.domain()
        .map(|url| domain_regex.is_match(url))
        .unwrap_or(false)
}

pub fn explore(options: &Options) {
    let mut visited: HashMap<String, ()> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(options.start_url.clone());

    while !queue.is_empty() {
        let current_url = queue.pop_back().unwrap();
        if visited.contains_key(current_url.as_str()) {
            continue;
        } else if current_url == options.final_url {
            break;
        }

        visited.insert(String::from(current_url.as_str()), ());
        println!("{}", current_url);

        let document = match download(&current_url) {
            Some(content) => content,
            None => continue
        };


        let mut urls = extract_from_html(&document);
        if let Some(regex) = &options.domain_regex {
            urls = urls.into_iter()
                .filter(|url| is_url_domain_ok(regex, url))
                .collect();
        };

        for url in urls {
            queue.push_back(url);
        }
    }
}