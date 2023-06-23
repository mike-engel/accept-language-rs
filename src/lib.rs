//! `accept-language` is a tiny library for parsing the Accept-Language header from browsers (as defined [here](https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html)).
//!
//! It's intended to be used in a web server that supports some level of internationalization (i18n),
//! but can be used anytime an Accept-Language header string is available.
//!
//! In order to help facilitate better i18n, a function is provided to return the intersection of
//! the languages the user prefers and the languages your application supports.
//!
//! # Example
//!
//! ```
//! use accept_language::{intersection, parse};
//!
//! let user_languages = parse("en-US, en-GB;q=0.5");
//! let common_languages = intersection("en-US, en-GB;q=0.5", &["en-US", "de", "en-GB"]);
//! ```
use std::cmp::Ordering;
use std::str;
use std::str::FromStr;

#[derive(Debug)]
struct Language {
    name: String,
    quality: f64,
}

impl Eq for Language {}

impl Ord for Language {
    fn cmp(&self, other: &Language) -> Ordering {
        if self.quality > other.quality {
            Ordering::Less
        } else if self.quality < other.quality {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Language {
    fn partial_cmp(&self, other: &Language) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Language {
    fn eq(&self, other: &Language) -> bool {
        self.quality == other.quality && self.name.to_lowercase() == other.name.to_lowercase()
    }
}

impl Language {
    fn new(tag: &str) -> Language {
        let tag_parts: Vec<&str> = tag.split(";").collect();
        let name = match tag_parts.len() {
            0 => String::from(""),
            _ => tag_parts[0].to_string(),
        };
        let quality = match tag_parts.len() {
            1 => 1.0,
            _ => Language::quality_with_default(tag_parts[1]),
        };

        Language {
            name: name,
            quality: quality,
        }
    }

    fn quality_with_default(raw_quality: &str) -> f64 {
        let quality_parts: Vec<&str> = raw_quality.split("=").collect();

        match quality_parts.len() {
            2 => match f64::from_str(quality_parts[1]) {
                Ok(q) => q,
                Err(_) => 0.0,
            },
            _ => 0.0,
        }
    }
}

/// Parse a raw Accept-Language header value into an ordered list of language tags.
/// This should return the exact same list as `window.navigator.languages` in supported browsers.
///
/// # Example
///
/// ```
/// use accept_language::parse;
///
/// let user_languages = parse("en-US, en-GB;q=0.5");
/// ```
pub fn parse(raw_languages: &str) -> Vec<String> {
    let stripped_languages = raw_languages.to_owned().replace(" ", "");
    let language_strings: Vec<&str> = stripped_languages.split(",").collect();
    let mut languages: Vec<Language> = language_strings.iter().map(|l| Language::new(l)).collect();

    languages.sort();

    languages
        .iter()
        .map(|ref l| l.name.to_owned())
        .filter(|l| !l.is_empty())
        .collect()
}

/// Compare an Accept-Language header value with your application's supported languages to find
/// the common languages that could be presented to a user.
///
/// # Example
///
/// ```
/// use accept_language::intersection;
///
/// let common_languages = intersection("en-US, en-GB;q=0.5", &["en-US", "de", "en-GB"]);
/// ```
pub fn intersection(raw_languages: &str, supported_languages: &[&str]) -> Vec<String> {
    let user_languages = parse(raw_languages);
    let intersection = user_languages
        .into_iter()
        .filter(|l| supported_languages.contains(&l.as_str()))
        .map(|l| l.to_string())
        .collect();

    intersection
}

#[cfg(test)]
mod tests {
    use super::{intersection, parse, Language};

    static MOCK_ACCEPT_LANGUAGE: &str = "en-US, de;q=0.7, jp;q=0.1";

    #[test]
    fn it_creates_a_new_language_from_a_string() {
        let language = Language::new("en-US;q=0.7");

        assert_eq!(
            language,
            Language {
                name: String::from("en-US"),
                quality: 0.7,
            }
        )
    }

    #[test]
    fn it_creates_a_new_language_from_a_string_with_lowercase_country() {
        let language = Language::new("en-us;q=0.7");

        assert_eq!(
            language,
            Language {
                name: String::from("en-US"),
                quality: 0.7,
            }
        )
    }

    #[test]
    fn it_creates_a_new_language_from_a_string_with_a_default_quality() {
        let language = Language::new("en-US");

        assert_eq!(
            language,
            Language {
                name: String::from("en-US"),
                quality: 1.0,
            }
        )
    }

    #[test]
    fn it_parses_quality() {
        let quality = Language::quality_with_default("q=0.5");

        assert_eq!(quality, 0.5)
    }

    #[test]
    fn it_parses_an_invalid_quality() {
        let quality = Language::quality_with_default("q=yolo");

        assert_eq!(quality, 0.0)
    }

    #[test]
    fn it_parses_a_valid_accept_language_header() {
        let user_languages = parse(MOCK_ACCEPT_LANGUAGE);

        assert_eq!(
            user_languages,
            vec![
                String::from("en-US"),
                String::from("de"),
                String::from("jp"),
            ]
        )
    }

    #[test]
    fn it_parses_an_empty_accept_language_header() {
        let user_languages = parse("");

        assert_eq!(user_languages.len(), 0)
    }

    #[test]
    fn it_parses_an_invalid_accept_language_header() {
        let user_languages_one = parse("q");
        let user_languages_two = parse(";q");
        let user_languages_three = parse("q-");
        let user_languages_four = parse("en;q=");

        assert_eq!(user_languages_one, vec![String::from("q")]);
        assert_eq!(user_languages_two.len(), 0);
        assert_eq!(user_languages_three, vec![String::from("q-")]);
        assert_eq!(user_languages_four, vec![String::from("en")])
    }

    #[test]
    fn it_sorts_languages_by_quality() {
        let user_languages = parse("en-US, de;q=0.1, jp;q=0.7");

        assert_eq!(
            user_languages,
            vec![
                String::from("en-US"),
                String::from("jp"),
                String::from("de"),
            ]
        )
    }

    #[test]
    fn it_returns_language_intersections() {
        let common_languages = intersection(MOCK_ACCEPT_LANGUAGE, &["en-US", "jp"]);

        assert_eq!(
            common_languages,
            vec![String::from("en-US"), String::from("jp")]
        )
    }

    #[test]
    fn it_returns_an_empty_array_when_no_intersections() {
        let common_languages = intersection(MOCK_ACCEPT_LANGUAGE, &["fr", "en-GB"]);

        assert_eq!(common_languages.len(), 0)
    }

    #[test]
    fn it_parses_traditional_chinese() {
        assert_eq!(parse("zh-Hant"), &["zh-Hant"]);
    }

    #[test]
    fn it_implements_case_insensitive_equality() {
        assert_eq!(Language::new("en-US"), Language::new("en-us"));
        assert_eq!(Language::new("en-US;q=0.7"), Language::new("en-us;q=0.7"));
        assert_ne!(Language::new("en"), Language::new("en-US"));
        assert_ne!(Language::new("en;q=0.7"), Language::new("en;q=0.8"));
        assert_ne!(Language::new("en;q=0.7"), Language::new("en-US;q=0.7"));
    }
}
