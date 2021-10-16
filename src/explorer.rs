use url::Url;
use regex::Regex;
use std::collections::{VecDeque, HashSet};
use crate::cli::Options;
use crate::document::Document;
use crate::url_extractor::extract_from_html;

fn is_url_domain_ok(domain_regex: &Regex, url: &str) -> bool {
    Url::parse(url)
        .map(|parsed_url| match parsed_url.domain() {
            Some(domain) => domain_regex.is_match(domain),
            _ => false
        })
        .ok()
        .unwrap_or(false)
}

fn extract_urls_from_site(url: &str, options: &Options) -> Option<Vec<String>> {
    let document = Document::download(url)?;

    let urls = extract_from_html(&document);
    Some(if let Some(regex) = &options.domain_regex {
        urls.into_iter()
            .filter(|url| is_url_domain_ok(regex, url))
            .collect()
    } else {
        urls
    })
}

pub fn explore(options: &Options) -> usize {
    let mut visited: HashSet<String> = HashSet::new();

    let mut queue = VecDeque::new();
    let mut auxillary_queue = VecDeque::new();
    let mut nesting_level = 0;

    queue.push_back(String::from(options.start_url.as_str()));

    'main: while !queue.is_empty() {
        while !queue.is_empty() {
            let current_url = queue.pop_front().unwrap();
            if visited.contains(&current_url) {
                continue;
            } else if current_url == options.final_url.as_str() {
                println!("{}", current_url);
                break 'main;
            }

            println!("{}", current_url);
            let result = extract_urls_from_site(&current_url, &options);
            visited.insert(current_url);

            if let Some(urls) = result {
                auxillary_queue.extend(urls.into_iter());
            }
        }
        
        println!();
        nesting_level += 1;
        queue.append(&mut auxillary_queue);
    }

    nesting_level
}