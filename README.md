# fileinput.rs

Read from multiple input streams like a cool CLI tool should.

[![Build Status](https://travis-ci.org/spladug/fileinput.rs.svg?branch=master)](https://travis-ci.org/spladug/fileinput.rs) [![crates.io status](https://img.shields.io/crates/v/fileinput.svg)](https://crates.io/crates/fileinput)

## Example

```rust
use std::io::{BufRead,BufReader};
use fileinput::FileInput;

let filenames = vec!["testdata/1", "testdata/2"];
let fileinput = FileInput::new(&filenames);
let mut reader = BufReader::new(fileinput);

for line in reader.lines() {
    println!("{}", line.unwrap());
}
```

## Documentation

The (minimal) API is documented:
https://www.spladug.net/rust/fileinput/index.html

## Installation

This package is on crates.io.

```toml
[dependencies]
fileinput = "*"
```

## License

This library is licensed under the MIT license, see [LICENSE](./LICENSE).

## Code signing

New commits in this repository are signed with my GPG key which [can be found
at keybase.io/spladug](https://keybase.io/spladug).
