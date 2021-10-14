use soup::prelude::*;

pub fn extract_from_html(html_file_content: &str) -> Vec<String> {
    let soup = Soup::new(html_file_content);
    
    soup.tag("a")
        .find_all()
        .map(|t| t.get("href"))
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect()
}
