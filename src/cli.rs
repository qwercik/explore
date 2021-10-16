use structopt::StructOpt;
use url::Url;
use regex::Regex;

#[derive(Debug, StructOpt)]
#[structopt(
    name="explore",
    about="Explore the WWW and find the shortest path between two HTML documents",
    author="Eryk Andrzejewski"
)]
pub struct Options {
    #[structopt(
        help="URL from which you would like to start exploring",
        parse(try_from_str=Url::parse)
    )]
    pub start_url: Url,

    #[structopt(
        help="URL to which you would like to find a path, beginning from the start URL",
        parse(try_from_str=Url::parse)
    )]
    pub final_url: Url,

    #[structopt(
        short="d",
        long="domain",
        help="Regex for filtering URLs",
        parse(try_from_str=Regex::new)
    )]
    pub domain_regex: Option<Regex>,

    #[structopt(
        short="v",
        long="verbose",
        help="Print out all visited URLs"
    )]
    pub verbose: bool
}

impl Options {
    pub fn get() -> Self {
        Options::from_args()
    }
}