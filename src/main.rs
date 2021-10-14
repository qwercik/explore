mod cli;

use cli::Options;

fn main() {
    let options = Options::get();
    println!("{:#?}", options);
}
