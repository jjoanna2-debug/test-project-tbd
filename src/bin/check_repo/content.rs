// Copyright 2026 Jean-Claude Joanna
// SPDX-License-Identifier: Apache-2.0

use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;

pub(crate) const MAX_TEXT_FILE_BYTES: u64 = 4 * 1024 * 1024;
const BINARY_SAMPLE_BYTES: usize = 64 * 1024;

const BINARY_SUFFIXES: &[&str] = &[
    "7z", "a", "avi", "bmp", "class", "dmg", "doc", "docx", "dylib", "eot", "exe", "flac", "gif",
    "gz", "ico", "jar", "jpeg", "jpg", "m4a", "mov", "mp3", "mp4", "o", "otf", "pdf", "png", "ppt",
    "pptx", "so", "sqlite", "sqlite3", "tar", "tiff", "ttf", "wav", "wasm", "webm", "webp", "woff",
    "woff2", "xls", "xlsx", "zip",
];

pub(crate) enum ScannableContent {
    Text {
        content: String,
        warning: Option<String>,
    },
    Binary,
}

pub(crate) fn read_required_text(path: &Path, display_path: &str) -> Result<String, String> {
    let metadata = fs::metadata(path)
        .map_err(|error| format!("{display_path}: metadata could not be read: {error}"))?;
    if metadata.len() > MAX_TEXT_FILE_BYTES {
        return Err(format!("{display_path}: exceeds the 4 MiB text scan limit"));
    }

    let bytes = fs::read(path)
        .map_err(|error| format!("{display_path}: could not be read: {error}"))?;
    String::from_utf8(bytes).map_err(|_| format!("{display_path}: must be readable as UTF-8"))
}

pub(crate) fn read_repository_content(
    path: &Path,
    display_path: &str,
) -> Result<ScannableContent, String> {
    let metadata = fs::metadata(path)
        .map_err(|error| format!("{display_path}: metadata could not be read: {error}"))?;
    let declared_binary = has_binary_suffix(path);

    let sample = read_sample(path)
        .map_err(|error| format!("{display_path}: could not be sampled: {error}"))?;
    let sample_kind = classify_sample(&sample);

    if declared_binary {
        return classify_declared_binary(path, display_path, metadata.len(), sample_kind);
    }

    if matches!(sample_kind, SampleKind::KnownBinary | SampleKind::BinaryLike) {
        return Err(format!(
            "{display_path}: contains binary data without a recognized binary extension"
        ));
    }
    if matches!(sample_kind, SampleKind::InvalidText) {
        return Err(format!("{display_path}: must be readable as UTF-8"));
    }
    if metadata.len() > MAX_TEXT_FILE_BYTES {
        return Err(format!("{display_path}: exceeds the 4 MiB text scan limit"));
    }

    Ok(ScannableContent::Text {
        content: read_full_text(path, display_path)?,
        warning: None,
    })
}

fn classify_declared_binary(
    path: &Path,
    display_path: &str,
    size: u64,
    sample_kind: SampleKind,
) -> Result<ScannableContent, String> {
    match sample_kind {
        SampleKind::KnownBinary | SampleKind::BinaryLike => Ok(ScannableContent::Binary),
        SampleKind::InvalidText => Err(format!(
            "{display_path}: uses a binary extension but contains invalid UTF-8 text-like data"
        )),
        SampleKind::Text => {
            if size > MAX_TEXT_FILE_BYTES {
                return Err(format!(
                    "{display_path}: contains UTF-8 text behind a binary extension but exceeds the 4 MiB text scan limit"
                ));
            }
            Ok(ScannableContent::Text {
                content: read_full_text(path, display_path)?,
                warning: Some(format!(
                    "{display_path}: uses a binary extension but contains UTF-8 text; content was scanned"
                )),
            })
        }
    }
}

fn read_sample(path: &Path) -> io::Result<Vec<u8>> {
    let mut sample = Vec::with_capacity(BINARY_SAMPLE_BYTES);
    File::open(path)?
        .take(BINARY_SAMPLE_BYTES as u64)
        .read_to_end(&mut sample)?;
    Ok(sample)
}

fn read_full_text(path: &Path, display_path: &str) -> Result<String, String> {
    let bytes = fs::read(path)
        .map_err(|error| format!("{display_path}: could not be read: {error}"))?;
    String::from_utf8(bytes).map_err(|_| format!("{display_path}: must be readable as UTF-8"))
}

fn has_binary_suffix(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
        .is_some_and(|extension| BINARY_SUFFIXES.contains(&extension.as_str()))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SampleKind {
    Text,
    KnownBinary,
    BinaryLike,
    InvalidText,
}

fn classify_sample(sample: &[u8]) -> SampleKind {
    if has_known_binary_magic(sample) {
        return SampleKind::KnownBinary;
    }
    if looks_binary(sample) {
        return SampleKind::BinaryLike;
    }
    if std::str::from_utf8(sample).is_ok() {
        SampleKind::Text
    } else {
        SampleKind::InvalidText
    }
}

fn looks_binary(sample: &[u8]) -> bool {
    if sample.is_empty() {
        return false;
    }
    if sample.contains(&0) {
        return true;
    }

    let controls = sample
        .iter()
        .filter(|byte| **byte < 0x20 && !matches!(**byte, b'\n' | b'\r' | b'\t' | 0x0c))
        .count();
    controls.saturating_mul(20) > sample.len()
}

fn has_known_binary_magic(bytes: &[u8]) -> bool {
    const PREFIXES: &[&[u8]] = &[
        b"%PDF-",
        b"\x89PNG\r\n\x1a\n",
        b"GIF87a",
        b"GIF89a",
        b"PK\x03\x04",
        b"PK\x05\x06",
        b"PK\x07\x08",
        b"\x1f\x8b",
        b"7z\xbc\xaf'\x1c",
        b"\x7fELF",
        b"MZ",
        b"\0asm",
        b"SQLite format 3\0",
        b"\xca\xfe\xba\xbe",
        b"OTTO",
        b"RIFF",
        b"ID3",
        b"fLaC",
        b"OggS",
    ];
    if PREFIXES.iter().any(|prefix| bytes.starts_with(prefix)) {
        return true;
    }
    if bytes.starts_with(&[0xff, 0xd8, 0xff]) {
        return true;
    }
    if bytes.starts_with(&[0x00, 0x01, 0x00, 0x00]) {
        return true;
    }
    if bytes.len() > 262 && bytes.get(257..262) == Some(b"ustar") {
        return true;
    }

    matches!(
        bytes.get(..4),
        Some([0xfe, 0xed, 0xfa, 0xce]
            | [0xfe, 0xed, 0xfa, 0xcf]
            | [0xce, 0xfa, 0xed, 0xfe]
            | [0xcf, 0xfa, 0xed, 0xfe]
            | [0xca, 0xfe, 0xba, 0xbe])
    )
}

#[cfg(test)]
mod tests {
    use super::{SampleKind, classify_sample, has_binary_suffix};
    use std::path::Path;

    #[test]
    fn recognizes_binary_extensions_case_insensitively() {
        assert!(has_binary_suffix(Path::new("archive.ZIP")));
        assert!(has_binary_suffix(Path::new("module.WASM")));
        assert!(!has_binary_suffix(Path::new("README.md")));
    }

    #[test]
    fn classifies_common_binary_signatures() {
        assert_eq!(classify_sample(b"%PDF-1.7\n"), SampleKind::KnownBinary);
        assert_eq!(
            classify_sample(b"\x89PNG\r\n\x1a\nrest"),
            SampleKind::KnownBinary
        );
        assert_eq!(classify_sample(b"PK\x03\x04rest"), SampleKind::KnownBinary);
    }

    #[test]
    fn distinguishes_text_binary_and_invalid_text() {
        assert_eq!(classify_sample(b"plain UTF-8 text\n"), SampleKind::Text);
        assert_eq!(classify_sample(b"abc\0def"), SampleKind::BinaryLike);
        assert_eq!(classify_sample(&[b'a', 0xff, b'b']), SampleKind::InvalidText);
    }
}
