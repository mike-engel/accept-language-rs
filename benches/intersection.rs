#![feature(test)]
extern crate accept_language;
extern crate test;

#[cfg(test)]
mod benches {
    use super::accept_language::*;
    use test::Bencher;

    static MOCK_ACCEPT_LANGUAGE: &str = "en-US, de;q=0.7, zh-Hant, jp;q=0.1";
    static AVIALABLE_LANGUAGES: &[&str] = &[
        "aa", "ab", "ae", "af", "ak", "am", "an", "ar", "as", "av", "ay", "az", "ba", "be", "bg",
        "bh", "bi", "bm", "bn", "bo", "br", "bs", "ca", "ce", "ch", "co", "cr", "cs", "cu", "cv",
        "cy", "da", "de", "dv", "dz", "ee", "el", "en", "en-UK", "en-US", "eo", "es", "es-ar",
        "et", "eu", "fa", "ff", "fi", "fj", "fo", "fr", "fy", "ga", "gd", "gl", "gn", "gu", "gv",
        "gv", "ha", "he", "hi", "ho", "hr", "ht", "hu", "hy", "hz", "ia", "id", "ie", "ig", "ii",
        "ii", "ik", "in", "io", "is", "it", "iu", "ja", "jp", "jv", "ka", "kg", "ki", "kj", "kk",
        "kl", "kl", "km", "kn", "ko", "kr", "ks", "ku", "kv", "kw", "ky", "la", "lb", "lg", "li",
        "ln", "lo", "lt", "lu", "lv", "mg", "mh", "mi", "mk", "ml", "mn", "mo", "mr", "ms", "mt",
        "my", "na", "nb", "nd", "ne", "ng", "nl", "nn", "no", "nr", "nv", "ny", "oc", "oj", "om",
        "or", "os", "pa", "pi", "pl", "ps", "pt", "qu", "rm", "rn", "ro", "ru", "rw", "sa", "sd",
        "se", "sg", "sh", "si", "sk", "sl", "sm", "sn", "so", "sq", "sr", "ss", "ss", "st", "su",
        "sv", "sw", "ta", "te", "tg", "th", "ti", "tk", "tl", "tn", "to", "tr", "ts", "tt", "tw",
        "ty", "ug", "uk", "ur", "uz", "ve", "vi", "vo", "wa", "wo", "xh", "yi", "yo", "za", "zh",
        "zh-Hans", "zh-Hant", "zu",
    ];

    #[bench]
    fn bench_intersections(b: &mut Bencher) {
        b.iter(|| intersection(MOCK_ACCEPT_LANGUAGE, AVIALABLE_LANGUAGES));
    }

    #[bench]
    fn bench_intersections_ordered(b: &mut Bencher) {
        b.iter(|| intersection_ordered(MOCK_ACCEPT_LANGUAGE, AVIALABLE_LANGUAGES));
    }
}
