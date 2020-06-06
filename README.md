# reddit-dl
Reddit images downloader in Rust

Inspired by https://github.com/darkmtr/reddit-downloader

This is intended to be my first project which uses rust async/await capabilities to concurrently download images from Reddit.

## Usage
<pre>
USAGE:
    reddit-dl [OPTIONS] <subreddit>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dir <DIR>                    Directory to save downloaded images (default is $HOME/Pictures/memes)
    -l, --limit <LIMIT>                Max number of images to download (defaults to 50)
    -m, --max_connections <MAXCONN>    Maximum number of simultaneous downloads (defaults to 10)

ARGS:
    <subreddit>    subreddit name
</pre>
