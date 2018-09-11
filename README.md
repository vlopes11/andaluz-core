## Andaluz Core [![Build Status](https://travis-ci.org/vlopes11/andaluz-core.svg?branch=master)](https://travis-ci.org/vlopes11/andaluz-core)

Library to solve n-queens problem written in Rust

### Usage

#### Command-line interface

```
$ git clone https://github.com/vlopes11/andaluz-core
$ cd andaluz-core
$ make release
$ ./cli/target/release/andaluz-cli -c 8
```

#### WebAssembly

Example with firefox
```
$ git clone https://github.com/vlopes11/andaluz-core
$ cd andaluz-core
$ make release
$ firefox wasm/site/index.html
```

### TODO

* Implement prime horses attack heuristics
