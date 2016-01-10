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

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

## Code signing

New commits in this repository are signed with my GPG key which [can be found
at keybase.io/spladug](https://keybase.io/spladug).
