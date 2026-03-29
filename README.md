# facilguide

[![Crates.io](https://img.shields.io/crates/v/facilguide.svg)](https://crates.io/crates/facilguide)
[![docs.rs](https://img.shields.io/docsrs/facilguide)](https://docs.rs/facilguide)

Internationalization helpers for multilingual content, built for the Facil.guide project -- a tech education platform that publishes step-by-step guides for seniors in five languages: English, Spanish, French, Portuguese, and Italian.

This crate provides locale detection, string lookup tables, and URL routing utilities so that Rust services can serve the right translation without pulling in a full i18n framework.

## Installation

```toml
[dependencies]
facilguide = "0.1.2"
```

## Usage

### Supported Locales

The `Locale` enum represents the five supported languages. Parse from a two-letter code or extract from a URL path prefix:

```rust
use facilguide::{Locale, parse_locale};

let locale = parse_locale("es").unwrap();
assert_eq!(locale, Locale::Es);
assert_eq!(locale.native_name(), "Espanol");
```

### URL Routing

Given a path like `/fr/guides/smartphone-basics`, extract the locale and the remaining path in one call:

```rust
use facilguide::split_locale_path;

let (locale, rest) = split_locale_path("/pt/guides/email-setup");
assert_eq!(locale.code(), "pt");
assert_eq!(rest, "/guides/email-setup");
```

### Translation Lookup

Register key-value pairs per locale and retrieve them at render time. Missing keys fall back to the English value:

```rust
use facilguide::{TranslationTable, Locale};

let mut table = TranslationTable::new();
table.insert("greeting", Locale::En, "Welcome");
table.insert("greeting", Locale::Es, "Bienvenido");
table.insert("greeting", Locale::Fr, "Bienvenue");

assert_eq!(table.get("greeting", Locale::Fr), "Bienvenue");
assert_eq!(table.get("greeting", Locale::It), "Welcome"); // fallback
```

### Guide Metadata

Represent a single guide entry with its language and difficulty level:

```rust
use facilguide::{Guide, Locale, Difficulty};

let guide = Guide {
    slug: "wifi-connect".into(),
    title: "Como conectar al WiFi".into(),
    locale: Locale::Es,
    difficulty: Difficulty::Beginner,
};
```

## What Facil.guide Covers

Facil.guide publishes plain-language technology guides aimed at older adults. Topics range from smartphone basics and email setup to video calls, app stores, online banking safety, and social media. Every guide is written at a beginner-friendly reading level and translated into all five supported languages. The difficulty scale (Beginner, Intermediate, Advanced) helps readers find content matched to their comfort level.

## Links

- [Facil.guide](https://facil.guide) -- multilingual tech guides for seniors
- [Source Code](https://github.com/arnaudleroy-studio/facilguide-rust)

## License

MIT
