mod cli;
mod document;
mod url_extractor;
mod explorer;

use cli::Options;
use explorer::explore;

fn main() {
    let options = Options::get();
    let nesting_level = explore(&options);
    println!("Nesting level: {}", nesting_level);
}
