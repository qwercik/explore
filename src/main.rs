mod cli;
mod document;
mod explorer;
mod url_extractor;

use cli::Options;
use explorer::explore;

fn main() {
    let options = Options::get();
    let nesting_level = explore(&options);

    if options.verbose {
        println!("Nesting level: {}", nesting_level);
    } else {
        println!("{}", nesting_level);
    }
}
