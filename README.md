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

Example with firefox
```
$ git clone https://github.com/vlopes11/andaluz-core
$ cd andaluz-core
$ make release
$ firefox "`pwd`/site/index.html"
```

### TODO

* Implement prime horses attack heuristics
