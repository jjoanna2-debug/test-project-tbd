// Copyright 2026 Jean-Claude Joanna
// SPDX-License-Identifier: Apache-2.0

use std::env;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum OutputFormat {
    Text,
    Json,
    GitHub,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum CliAction {
    Run(OutputFormat),
    Help,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Finding {
    pub(crate) path: Option<String>,
    pub(crate) line: Option<usize>,
    pub(crate) message: String,
}

pub(crate) fn parse_cli() -> Result<CliAction, String> {
    parse_args(env::args().skip(1), default_output_format())
}

pub(crate) fn help_text() -> &'static str {
    "Usage: check_repo [--format text|json|github]\n\nFormats:\n  text    Human-readable output\n  json    Deterministic machine-readable findings\n  github  GitHub Actions workflow command annotations\n"
}

pub(crate) fn findings_from_raw(raw: impl IntoIterator<Item = String>) -> Vec<Finding> {
    raw.into_iter()
        .map(|value| Finding::from_raw(&value))
        .collect()
}

pub(crate) fn render(format: OutputFormat, findings: &[Finding]) {
    match format {
        OutputFormat::Text => render_text(findings),
        OutputFormat::Json => render_json(findings),
        OutputFormat::GitHub => render_github(findings),
    }
}

impl Finding {
    fn from_raw(raw: &str) -> Self {
        if let Some((path, line, message)) = split_path_line(raw) {
            return Self {
                path: Some(path.to_owned()),
                line: Some(line),
                message: message.to_owned(),
            };
        }

        if let Some((prefix, message)) = raw.split_once(':')
            && looks_like_path(prefix)
        {
            return Self {
                path: Some(prefix.to_owned()),
                line: None,
                message: message.trim_start().to_owned(),
            };
        }

        if let Some((prefix, _)) = raw.split_once(' ')
            && looks_like_path(prefix)
        {
            return Self {
                path: Some(prefix.to_owned()),
                line: None,
                message: raw[prefix.len()..].trim_start().to_owned(),
            };
        }

        Self {
            path: None,
            line: None,
            message: raw.to_owned(),
        }
    }
}

fn parse_args<I>(args: I, default: OutputFormat) -> Result<CliAction, String>
where
    I: IntoIterator<Item = String>,
{
    let mut args = args.into_iter();
    let Some(first) = args.next() else {
        return Ok(CliAction::Run(default));
    };

    if matches!(first.as_str(), "-h" | "--help") {
        if args.next().is_some() {
            return Err("--help does not accept additional arguments".to_owned());
        }
        return Ok(CliAction::Help);
    }

    if first != "--format" {
        return Err(format!("unknown argument: {first}"));
    }

    let value = args
        .next()
        .ok_or_else(|| "--format requires text, json, or github".to_owned())?;
    if args.next().is_some() {
        return Err("only one --format value may be supplied".to_owned());
    }

    let format = match value.as_str() {
        "text" => OutputFormat::Text,
        "json" => OutputFormat::Json,
        "github" => OutputFormat::GitHub,
        _ => return Err(format!("unsupported output format: {value}")),
    };
    Ok(CliAction::Run(format))
}

fn default_output_format() -> OutputFormat {
    if env::var_os("GITHUB_ACTIONS").is_some_and(|value| value == "true") {
        OutputFormat::GitHub
    } else {
        OutputFormat::Text
    }
}

fn split_path_line(raw: &str) -> Option<(&str, usize, &str)> {
    let (path, rest) = raw.split_once(':')?;
    if !looks_like_path(path) {
        return None;
    }

    let digit_count = rest
        .as_bytes()
        .iter()
        .take_while(|byte| byte.is_ascii_digit())
        .count();
    if digit_count == 0 {
        return None;
    }

    let line = rest[..digit_count].parse().ok()?;
    let message = rest[digit_count..]
        .strip_prefix(':')
        .unwrap_or(&rest[digit_count..])
        .trim_start();
    Some((path, line, message))
}

fn looks_like_path(value: &str) -> bool {
    !value.is_empty()
        && !value.chars().any(char::is_whitespace)
        && (value.contains('/') || value.contains('.') || value.starts_with('.'))
}

fn render_text(findings: &[Finding]) {
    if findings.is_empty() {
        println!("Repository check passed.");
        return;
    }

    println!("Repository check failed:");
    for finding in findings {
        match (&finding.path, finding.line) {
            (Some(path), Some(line)) => println!("- {path}:{line} {}", finding.message),
            (Some(path), None) => println!("- {path}: {}", finding.message),
            (None, _) => println!("- {}", finding.message),
        }
    }
}

fn render_json(findings: &[Finding]) {
    print!("{{\"ok\":{},\"findings\":[", findings.is_empty());
    for (index, finding) in findings.iter().enumerate() {
        if index > 0 {
            print!(",");
        }
        print!("{{\"path\":");
        match &finding.path {
            Some(path) => print!("\"{}\"", json_escape(path)),
            None => print!("null"),
        }
        print!(",\"line\":");
        match finding.line {
            Some(line) => print!("{line}"),
            None => print!("null"),
        }
        print!(",\"message\":\"{}\"}}", json_escape(&finding.message));
    }
    println!("]}}");
}

fn render_github(findings: &[Finding]) {
    if findings.is_empty() {
        println!("::notice::Repository check passed.");
        return;
    }

    for finding in findings {
        let message = github_message_escape(&finding.message);
        match (&finding.path, finding.line) {
            (Some(path), Some(line)) => println!(
                "::error file={},line={}::{message}",
                github_property_escape(path),
                line
            ),
            (Some(path), None) => {
                println!("::error file={}::{message}", github_property_escape(path));
            }
            (None, _) => println!("::error::{message}"),
        }
    }
}

fn json_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            character if character < ' ' => {
                use std::fmt::Write as _;
                let _ = write!(escaped, "\\u{:04x}", u32::from(character));
            }
            character => escaped.push(character),
        }
    }
    escaped
}

fn github_message_escape(value: &str) -> String {
    value
        .replace('%', "%25")
        .replace('\r', "%0D")
        .replace('\n', "%0A")
}

fn github_property_escape(value: &str) -> String {
    github_message_escape(value)
        .replace(':', "%3A")
        .replace(',', "%2C")
}

#[cfg(test)]
mod tests {
    use super::{CliAction, Finding, OutputFormat, json_escape, parse_args};

    #[test]
    fn parses_supported_output_formats() {
        assert_eq!(
            parse_args(
                ["--format".to_owned(), "json".to_owned()],
                OutputFormat::Text
            ),
            Ok(CliAction::Run(OutputFormat::Json))
        );
        assert_eq!(
            parse_args(Vec::<String>::new(), OutputFormat::GitHub),
            Ok(CliAction::Run(OutputFormat::GitHub))
        );
    }

    #[test]
    fn rejects_unknown_cli_arguments() {
        assert!(parse_args(["--wat".to_owned()], OutputFormat::Text).is_err());
        assert!(
            parse_args(
                ["--format".to_owned(), "xml".to_owned()],
                OutputFormat::Text
            )
            .is_err()
        );
    }

    #[test]
    fn extracts_structured_path_and_line() {
        assert_eq!(
            Finding::from_raw("src/main.rs:12 unsafe construct"),
            Finding {
                path: Some("src/main.rs".to_owned()),
                line: Some(12),
                message: "unsafe construct".to_owned(),
            }
        );
        assert_eq!(
            Finding::from_raw("README.md: required file is missing"),
            Finding {
                path: Some("README.md".to_owned()),
                line: None,
                message: "required file is missing".to_owned(),
            }
        );
    }

    #[test]
    fn escapes_json_control_characters() {
        assert_eq!(json_escape("a\"b\\c\n"), "a\\\"b\\\\c\\n");
    }
}
