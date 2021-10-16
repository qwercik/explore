# explore
Find shortest path between two web resources.

## About
I decided to create this project because some day I started to wonder: In how many clicks can I go from one to another article on Wikipedia?

This app takes two URLs (they don't have to belong to Wikipedia; the only requirement is they have to refer to HTML documents, not any media files) and use BFS algorithm to solve the problem. Each page is downloaded and next level URLs are extracted from it.

At the end of the process app prints the number of clicks (depth of the tree).

## Build
```sh
git clone https://github.com/qwercik/explore
cd explore
cargo build --release

# To run app type
./target/release/explore <run options...>
```

## Usage
To use app you have to give two URLs as parameters. The first URL is start URL, and the following is end URL.
```sh
explore 'https://en.wikipedia.org/wiki/Prolog' 'https://en.wikipedia.org/wiki/Poland'
```

You can also specify URL `--domain` regex (e.g. to avoid visiting other sites than Wikipedia).
```sh
explore 'https://en.wikipedia.org/wiki/Prolog' 'https://en.wikipedia.org/wiki/Poland' --domain '^pl.wikipedia.org$'
```

If you would like to see which pages app visits, you can use `--verbose` option.
```sh
explore 'https://en.wikipedia.org/wiki/Prolog' 'https://en.wikipedia.org/wiki/Poland' --verbose
```

Unfortunately, at the moment app doesn't store URL's tree.

To get know more about supported options use `--help`.
```sh
explore --help
```

## To do
- better up performance
- provide an option for storing URL's tree
- use threads to do some operations paralell
- create a distributed system for solving more complex instances