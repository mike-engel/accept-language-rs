# 1.2.1

> 2018-09-22

#### Bug fixes

- accept-language-rs is now testing with cargo-fuzz. You can find the test results in the [README.md](README.md) file
- Compile and test with the latest version of stable rust

# 1.2.0

> 2018-07-07

#### Minor changes

- Don't auto-capitalize language codes [[#3](https://github.com/mike-engel/accept-language-rs/pull/3)]

# 1.1.1

> 2018-06-23

#### Bug fixes

- Prevent the thread from panicking when an invalid input is provided
- Compile and test with the latest version of rust

# 1.1.0

> 2017-08-30

Capitalization updates

#### Minor changes

- languages that come accross as all lowercase (e.g. "en-us") will be properly capitalized (e.g. "en-US")
- Minor refactor to avoid mutability!
- Remove a cargo category that doesn't make sense anymore

# 1.0.1

> 2017-08-25

Metadata and Readme updates

#### Bug fixes

- Fix the repo url in the cargo.toml file: #1 5b3bb5fef68067829c4183d8fac1dae0ad2b4638
- Add language about the code of conduct from my other newer projects

# 1.0.0

> 2017-05-25

The initial release of accept-language. Essentially two features:

- Parse an Accept-Language header into an ordered array of language tags
- Compare an Accept-Language header with your application's supported languages for more user friendly internationalization
