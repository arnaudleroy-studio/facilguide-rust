//! # facilguide
//!
//! Internationalization helpers for multilingual content. Built for
//! Facil.guide, a tech education platform publishing guides in five
//! languages for seniors.
//!
//! Homepage: <https://facil.guide>

use std::collections::HashMap;

/// Library version.
pub const VERSION: &str = "0.1.2";

/// Base URL for Facil.guide.
pub const BASE_URL: &str = "https://facil.guide";

// ---------------------------------------------------------------------------
// Locale
// ---------------------------------------------------------------------------

/// The five supported languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Locale {
    En,
    Es,
    Fr,
    Pt,
    It,
}

impl Locale {
    /// ISO 639-1 two-letter code.
    pub fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Es => "es",
            Self::Fr => "fr",
            Self::Pt => "pt",
            Self::It => "it",
        }
    }

    /// Name in the language itself.
    pub fn native_name(self) -> &'static str {
        match self {
            Self::En => "English",
            Self::Es => "Espanol",
            Self::Fr => "Francais",
            Self::Pt => "Portugues",
            Self::It => "Italiano",
        }
    }

    /// English name.
    pub fn english_name(self) -> &'static str {
        match self {
            Self::En => "English",
            Self::Es => "Spanish",
            Self::Fr => "French",
            Self::Pt => "Portuguese",
            Self::It => "Italian",
        }
    }
}

/// All supported locales.
pub const ALL_LOCALES: &[Locale] = &[
    Locale::En, Locale::Es, Locale::Fr, Locale::Pt, Locale::It,
];

/// Parse a two-letter language code into a [`Locale`].
pub fn parse_locale(code: &str) -> Option<Locale> {
    match code.to_lowercase().as_str() {
        "en" => Some(Locale::En),
        "es" => Some(Locale::Es),
        "fr" => Some(Locale::Fr),
        "pt" => Some(Locale::Pt),
        "it" => Some(Locale::It),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// URL routing
// ---------------------------------------------------------------------------

/// Split a URL path into its locale prefix and the remaining path.
///
/// If the first path segment is a recognised locale code, it is extracted.
/// Otherwise the path is returned unmodified with [`Locale::En`] as default.
///
/// ```
/// use facilguide::split_locale_path;
/// let (locale, rest) = split_locale_path("/fr/guides/wifi");
/// assert_eq!(locale.code(), "fr");
/// assert_eq!(rest, "/guides/wifi");
/// ```
pub fn split_locale_path(path: &str) -> (Locale, &str) {
    let trimmed = path.strip_prefix('/').unwrap_or(path);
    if let Some((prefix, _rest)) = trimmed.split_once('/') {
        if let Some(locale) = parse_locale(prefix) {
            return (locale, &path[prefix.len() + 1..]);
        }
    }
    (Locale::En, path)
}

// ---------------------------------------------------------------------------
// Difficulty
// ---------------------------------------------------------------------------

/// Guide difficulty level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Beginner     => "Beginner",
            Self::Intermediate => "Intermediate",
            Self::Advanced     => "Advanced",
        };
        f.write_str(label)
    }
}

// ---------------------------------------------------------------------------
// Guide
// ---------------------------------------------------------------------------

/// A single guide entry.
#[derive(Debug, Clone)]
pub struct Guide {
    pub slug: String,
    pub title: String,
    pub locale: Locale,
    pub difficulty: Difficulty,
}

// ---------------------------------------------------------------------------
// Translation table
// ---------------------------------------------------------------------------

/// Simple key-per-locale translation table with English fallback.
#[derive(Debug, Clone, Default)]
pub struct TranslationTable {
    entries: HashMap<String, HashMap<Locale, String>>,
}

impl TranslationTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a translated string for `key` in `locale`.
    pub fn insert(&mut self, key: &str, locale: Locale, value: &str) {
        self.entries
            .entry(key.to_owned())
            .or_default()
            .insert(locale, value.to_owned());
    }

    /// Retrieve the translation for `key` in `locale`.
    /// Falls back to English if the requested locale is missing.
    /// Returns the key itself if no translation exists at all.
    pub fn get<'a>(&'a self, key: &'a str, locale: Locale) -> &'a str {
        self.entries
            .get(key)
            .and_then(|map| {
                map.get(&locale).or_else(|| map.get(&Locale::En))
            })
            .map(|s| s.as_str())
            .unwrap_or(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locale_roundtrip() {
        for locale in ALL_LOCALES {
            assert_eq!(parse_locale(locale.code()), Some(*locale));
        }
    }

    #[test]
    fn split_path_with_locale() {
        let (loc, rest) = split_locale_path("/es/guides/email");
        assert_eq!(loc, Locale::Es);
        assert_eq!(rest, "/guides/email");
    }

    #[test]
    fn split_path_no_locale() {
        let (loc, rest) = split_locale_path("/about");
        assert_eq!(loc, Locale::En);
        assert_eq!(rest, "/about");
    }

    #[test]
    fn translation_fallback() {
        let mut t = TranslationTable::new();
        t.insert("ok", Locale::En, "OK");
        assert_eq!(t.get("ok", Locale::Fr), "OK"); // falls back to EN
    }

    #[test]
    fn translation_missing_key() {
        let t = TranslationTable::new();
        assert_eq!(t.get("missing", Locale::En), "missing");
    }
}
