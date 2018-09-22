# accept-language

> A tiny library for parsing the Accept-Language header from browsers (as defined [here](https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html))

# Usage

`accept-language` is intended to be used by a webserver, probably to decide which languages to serve up to the user based on their preferred language and the languages your application supports.

At it's most basic, it looks like this

```rust
extern crate accept_language;

use accept_language::{intersection, parse};

let user_languages = parse("en-US, en-GB;q=0.5");
let common_languages = intersection("en-US, en-GB;q=0.5", vec!["en-US", "de", "en-GB"]);
```

For more info and to view the full documentation, check them out on [docs.rs](https://docs.rs/accept-language).

# Stability

`accept-language` is fuzz tested with [`cargo-fuzz`](). As of `1.2.1`, these are the results of both fuzz tests for `parse` and `intersection` respectively.

**`parse`**

```sh
cargo fuzz run -O parse -- -max_total_time=60
...
Done 926619 runs in 61 second(s)
```

**`intersection`**

```sh
cargo fuzz run -O intersection -- -max_total_time=60
...
Done 846914 runs in 61 second(s)
```

# Contributing

Contributions are always welcome! If you found a bug, please submit an issue. If you'd like to submit a patch or feature, feel free to submit a pull request. [rustfmt](https://github.com/rust-lang-nursery/rustfmt) should be used to have consistent code formatting throughout the project.

# [Code of Conduct](CODE_OF_CONDUCT.md)

Please note that this project is released with a Contributor Code of Conduct. By participating in this project you agree to abide by its terms.

# [Changelog](CHANGELOG.md)

# [License](LICENSE.md)
