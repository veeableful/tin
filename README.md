Tin
===
A tiny one-off static file server that can be used as an alternative to Python's `SimpleHTTPServer` or `http.server`.

Install
-------
Pre-built binaries haven't been made available yet but if you have [Rust](https://rust-lang.org), you can get it from crates.io via `cargo install tin`.

Usage
-----
You can quickly serve your static site by going to the site's directory and type:  
`tin`

You can also change the port of the server by specifying the `-p` or `--port` option, like so:
`tin -p 8080`

If for some reason you want to specify the site directory, you can do:
`tin -d www/html`

License
-------
This project is GPLv2-licensed.
