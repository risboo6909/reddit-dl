# reddit-dl
Reddit images downloader written in Rust

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
    -d, --dir <DIR>                    Directory to save downloaded images (default is $HOME/Pictures/reddit)
    -l, --limit <LIMIT>                Max number of images to download (defaults to 50)
    -m, --max_connections <MAXCONN>    Maximum number of simultaneous downloads (defaults to 10)

ARGS:
    <subreddit>    subreddit name
</pre>

## Examples

Download 100 images from subreddit **memes** into $HOME/Pictures/reddit/memes directory:

`./reddit-dl memes`

Download 50 images from subreddit **dankmemes** into ./dankmemes directory:

`./reddit-dl dankmemes -l 50 -d darkmemes`
