mod cli;
mod downloader;
mod url_extractor;
mod explorer;

use cli::Options;
use explorer::explore;

fn main() {
    let options = Options::get();
    explore(&options.start_url, &options.final_url);
}
