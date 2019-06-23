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

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore -->
<table>
  <tr>
    <td align="center"><a href="https://www.mike-engel.com"><img src="https://avatars0.githubusercontent.com/u/464447?v=4" width="100px;" alt="Mike Engel"/><br /><sub><b>Mike Engel</b></sub></a><br /><a href="https://github.com/mike-engel/accept-language-rs/issues?q=author%3Amike-engel" title="Bug reports">🐛</a> <a href="#question-mike-engel" title="Answering Questions">💬</a> <a href="https://github.com/mike-engel/accept-language-rs/commits?author=mike-engel" title="Code">💻</a> <a href="https://github.com/mike-engel/accept-language-rs/commits?author=mike-engel" title="Documentation">📖</a> <a href="#design-mike-engel" title="Design">🎨</a> <a href="#ideas-mike-engel" title="Ideas, Planning, & Feedback">🤔</a> <a href="#infra-mike-engel" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a> <a href="#review-mike-engel" title="Reviewed Pull Requests">👀</a> <a href="https://github.com/mike-engel/accept-language-rs/commits?author=mike-engel" title="Tests">⚠️</a></td>
  </tr>
</table>

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!