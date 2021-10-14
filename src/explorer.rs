use url::Url;
use std::collections::{VecDeque, HashMap};
use crate::downloader::download;
use crate::url_extractor::extract_from_html;

pub fn explore(start_url: &Url, final_url: &Url) {
    let mut visited: HashMap<String, ()> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(start_url.clone());

    while !queue.is_empty() {
        let current_url = queue.pop_back().unwrap();
        if visited.contains_key(current_url.as_str()) {
            continue;
        } else if current_url == *final_url {
            break;
        }

        visited.insert(String::from(current_url.as_str()), ());
        println!("{}", current_url);

        let document = match download(&current_url) {
            Some(content) => content,
            None => continue
        };

        let urls = extract_from_html(&document);
        for url in urls {
            queue.push_back(url);
        }
    }
}