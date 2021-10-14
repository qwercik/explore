mod cli;
mod downloader;
mod url_extractor;

use cli::Options;
use downloader::download;
use url_extractor::extract_from_html;
use url::Url;

fn main() {
    let options = Options::get();
    let document = download(&options.start_url)
        .expect("downloading html");

    let urls = extract_from_html(&document);
    for url in urls {
        println!("{}", url);
    }
}
