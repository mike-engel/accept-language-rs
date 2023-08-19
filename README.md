# accept-language

> A tiny library for parsing the Accept-Language header from browsers (as defined [here](https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html))

# Usage

`accept-language` is intended to be used by a web server, probably to decide which languages to serve up to the user based on their preferred language and the languages your application supports.

At it's most basic, it looks like this

```rust
extern crate accept_language;

use accept_language::{intersection, parse};

let user_languages = parse("en-US, en-GB;q=0.5");
let common_languages = intersection("en-US, en-GB;q=0.5", vec!["en-US", "de", "en-GB"]);
```

For more info and to view the full documentation, check them out on [docs.rs](https://docs.rs/accept-language).

# Stability

`accept-language` is fuzz tested with [`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz) on every PR and push via GitHub actions.

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
    <td align="center"><a href="http://lukaskalbertodt.github.io/"><img src="https://avatars1.githubusercontent.com/u/7419664?v=4" width="100px;" alt="Lukas Kalbertodt"/><br /><sub><b>Lukas Kalbertodt</b></sub></a><br /><a href="https://github.com/mike-engel/accept-language-rs/commits?author=LukasKalbertodt" title="Code">💻</a></td>
    <td align="center"><a href="https://github.com/sstangl"><img src="https://avatars0.githubusercontent.com/u/171223?v=4" width="100px;" alt="Sean Stangl"/><br /><sub><b>Sean Stangl</b></sub></a><br /><a href="https://github.com/mike-engel/accept-language-rs/commits?author=sstangl" title="Code">💻</a> <a href="https://github.com/mike-engel/accept-language-rs/issues?q=author%3Asstangl" title="Bug reports">🐛</a> <a href="https://github.com/mike-engel/accept-language-rs/commits?author=sstangl" title="Tests">⚠️</a></td>
    <td align="center"><a href="https://kornel.ski"><img src="https://avatars0.githubusercontent.com/u/72159?v=4" width="100px;" alt="Kornel"/><br /><sub><b>Kornel</b></sub></a><br /><a href="https://github.com/mike-engel/accept-language-rs/commits?author=kornelski" title="Code">💻</a></td>
    <td align="center"><a href="https://daniellockyer.com"><img src="https://avatars2.githubusercontent.com/u/964245?v=4" width="100px;" alt="Daniel Lockyer"/><br /><sub><b>Daniel Lockyer</b></sub></a><br /><a href="https://github.com/mike-engel/accept-language-rs/issues?q=author%3Aneosilky" title="Bug reports">🐛</a></td>
    <td align="center"><a href="https://github.com/peter-scholtens"><img src="https://avatars.githubusercontent.com/u/7198614?v=4" width="100px;" alt="Peter C. S. Scholtens"/><br /><sub><b>Peter C. S. Scholtens</b></sub></a><br /><a href="https://github.com/mike-engel/accept-language-rs/issues?q=author%3Apeter-scholtens" title="Bug reports">🐛</a> <a href="https://github.com/mike-engel/accept-language-rs/commits?author=peter-scholtens" title="Code">💻</a></td>
  </tr>
</table>

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!