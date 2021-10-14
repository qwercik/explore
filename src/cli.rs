use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name="explore",
    about="Explore the WWW and find the shortest path between two HTML documents",
    author="Eryk Andrzejewski"
)]
pub struct Options {
    #[structopt(help="URL from which you would like to start exploring")]
    pub start_url: String,

    #[structopt(help="URL to which you would like to find a path, beginning from the start URL")]
    pub final_url: String
}

impl Options {
    pub fn get() -> Self {
        Options::from_args()
    }
}