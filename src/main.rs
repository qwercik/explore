use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name="explore",
    about="Explore the WWW and find the shortest path between two HTML documents",
    author="Eryk Andrzejewski"
)]
struct Options {
    #[structopt(help="URL from which you would like to start exploring")]
    start_url: String,

    #[structopt(help="URL to which you would like to find a path, beginning from the start URL")]
    final_url: String,
}

fn main() {
    let options = Options::from_args();
    println!("{:#?}", options);
}
