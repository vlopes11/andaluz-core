## Andaluz Core [![Build Status](https://travis-ci.org/vlopes11/andaluz-core.svg?branch=master)](https://travis-ci.org/vlopes11/andaluz-core)

Library to solve n-queens problem written in Rust

### Usage

#### Command-line interface

```
$ git clone https://github.com/vlopes11/andaluz-core
$ cd andaluz-core
$ make release
$ ./target/release/andaluz-core -c 8
```

#### WebAssembly

Some browsers won't recognize .wasm files as mime application/wasm. Either serve "web/dist/index.html" via a web server, so the proper mime headers will be set, or enforce this on your local client.

More information: https://trac.nginx.org/nginx/ticket/1606

Example with firefox (with proper MIME setup)
```
$ git clone https://github.com/vlopes11/andaluz-core
$ cd andaluz-core
$ make release
$ firefox "`pwd`/web/dist/index.html"
```

### TODO

* Implement prime horses attack heuristics
