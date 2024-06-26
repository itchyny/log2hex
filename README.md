# log2hex
[![CI Status](https://github.com/itchyny/log2hex/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/itchyny/log2hex/actions?query=branch:main)
[![release](https://img.shields.io/github/release/itchyny/log2hex/all.svg)](https://github.com/itchyny/log2hex/releases)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/itchyny/log2hex/blob/main/LICENSE)

Arbitrary place digits viewer of log(2) in base hex written in Rust.

```
 $ log2hex
0: b172 17f7 d1cf 79ab c9e3 b398 03f2 f6af
 $ log2hex 4
4: 17f7 d1cf 79ab c9e3 b398 03f2 f6af 40f3
 $ log2hex 8
8: d1cf 79ab c9e3 b398 03f2 f6af 40f3 4326
 $ log2hex 128
128: 3e96 ca16 224a e8c5 1acb da11 317c 387e
 $ log2hex 65536
65536: 2a18 16db 29dc edb7 5c69 e5fa 7af5 9d9f
 $ log2hex 1000000
1000000: 1848 9a94 06ec 9f80 4d3f 0ae1 af64 e6d5
```

## Installation
### Homebrew
```shell
brew install itchyny/tap/log2hex
```

### Build from source
```shell
git clone https://github.com/itchyny/log2hex
cd log2hex
cargo install --path .
```

## Author
itchyny (<https://github.com/itchyny>)

## License
This software is released under the MIT License, see LICENSE.
