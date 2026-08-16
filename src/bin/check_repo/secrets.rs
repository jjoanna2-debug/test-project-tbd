// Copyright 2026 Jean-Claude Joanna
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
#[path = "secrets/corpus.rs"]
mod corpus;

const PREFIXED_SECRET_RULES: &[(&str, &[&str], usize)] = &[
    (
        "GitHub token",
        &["ghp_", "gho_", "ghu_", "ghs_", "ghr_"],
        24,
    ),
    ("GitHub fine-grained token", &["github_pat_"], 24),
    (
        "GitLab token",
        &[
            "glpat-", "gloas-", "gldt-", "glrt-", "glrtr-", "glcbt-", "glptt-", "glft-", "glimt-",
            "glagent-", "glwt-", "glsoat-", "glffct-",
        ],
        16,
    ),
    ("OpenAI API key", &["sk-proj-", "sk-svcacct-"], 20),
    (
        "Slack token",
        &["xoxb-", "xoxp-", "xoxa-", "xoxr-", "xoxs-"],
        20,
    ),
    ("Stripe live secret key", &["sk_live_", "rk_live_"], 16),
    ("npm access token", &["npm_"], 30),
    ("Google API key", &["AIza"], 30),
    ("SendGrid API key", &["SG."], 40),
];

const STRONG_SECRET_KEYS: &[&str] = &[
    "access_token",
    "accesstoken",
    "api_key",
    "apikey",
    "auth_token",
    "authtoken",
    "bearer_token",
    "bearertoken",
    "client_secret",
    "clientsecret",
    "database_password",
    "db_password",
    "encryption_key",
    "master_key",
    "passphrase",
    "password",
    "passwd",
    "private_key",
    "privatekey",
    "pwd",
    "refresh_token",
    "refreshtoken",
    "secret",
    "secret_key",
    "secretkey",
    "session_secret",
    "session_token",
    "signing_key",
    "signingkey",
    "token",
    "webhook_secret",
    "webhooksecret",
];

const MODERATE_SECRET_KEYS: &[&str] = &[
    "authorization",
    "connection_string",
    "credentials",
    "credential",
    "database_url",
    "db_url",
    "sas_token",
    "service_account_key",
];

const KEY_PLACEHOLDER_MARKERS: &[&str] = &[
    "dummy",
    "example",
    "fake",
    "fixture",
    "mock",
    "placeholder",
    "sample",
    "test",
];

const VALUE_PLACEHOLDER_MARKERS: &[&str] = &[
    "changeme",
    "dummy-secret",
    "dummy_secret",
    "example",
    "fake-secret",
    "fake_secret",
    "not-a-real",
    "not_real",
    "notsecret",
    "placeholder",
    "redacted",
    "replace-me",
    "replace_me",
    "sample-secret",
    "sample_secret",
    "test-only",
    "test_only",
    "your-api",
    "your-secret",
    "your-token",
    "your_api",
    "your_secret",
    "your_token",
];

const PRIVATE_KEY_PREFIX: &str = "-----BEGIN ";
const PRIVATE_KEY_WORDS: [&str; 2] = ["PRIVATE ", "KEY-----"];
const SECRET_ASSIGNMENT_THRESHOLD: u8 = 70;
const MAX_SECRET_VALUE_BYTES: usize = 4_096;
const MAX_SECRET_VALUE_LINES: usize = 32;

pub(crate) fn check_secret_content(relative_path: &str, content: &str, failures: &mut Vec<String>) {
    let private_key_suffix = PRIVATE_KEY_WORDS.concat();
    if content.contains(PRIVATE_KEY_PREFIX) && content.contains(&private_key_suffix) {
        failures.push(format!(
            "{relative_path} appears to contain private key block"
        ));
    }

    for &(label, prefixes, minimum_tail_len) in PREFIXED_SECRET_RULES {
        if contains_prefixed_token(content, prefixes, minimum_tail_len) {
            failures.push(format!("{relative_path} appears to contain {label}"));
        }
    }

    if contains_aws_access_key(content) {
        failures.push(format!(
            "{relative_path} appears to contain AWS access key id"
        ));
    }

    if let Some((line_number, score)) = highest_secret_assignment_score(content) {
        failures.push(format!(
            "{relative_path}:{line_number} appears to contain likely secret assignment (signal score {score}/100)"
        ));
    }
}

#[cfg(test)]
fn secret_signal_score(content: &str) -> u8 {
    if PREFIXED_SECRET_RULES
        .iter()
        .any(|(_, prefixes, minimum_tail_len)| {
            contains_prefixed_token(content, prefixes, *minimum_tail_len)
        })
        || contains_aws_access_key(content)
    {
        return 100;
    }

    highest_secret_assignment_score(content).map_or(0, |(_, score)| score)
}

fn contains_prefixed_token(content: &str, prefixes: &[&str], minimum_tail_len: usize) -> bool {
    content
        .split(|character: char| {
            !(character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.'))
        })
        .any(|token| {
            prefixes.iter().any(|prefix| {
                token
                    .strip_prefix(prefix)
                    .is_some_and(|tail| is_plausible_provider_token(token, tail, minimum_tail_len))
            })
        })
}

fn is_plausible_provider_token(token: &str, tail: &str, minimum_tail_len: usize) -> bool {
    tail.len() >= minimum_tail_len
        && token.len() <= 512
        && tail
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'))
        && distinct_ascii_count(tail) >= 8
        && !is_repeated_pattern(tail)
        && !has_monotonic_sequence(tail, 8)
        && !is_provider_placeholder(tail)
        && !is_placeholder_value(token)
}

fn is_provider_placeholder(tail: &str) -> bool {
    const PLACEHOLDER_WORDS: &[&str] = &[
        "changeme",
        "dummy",
        "example",
        "fake",
        "placeholder",
        "redacted",
        "replace",
        "sample",
        "test",
        "your",
    ];

    tail.split(['_', '-', '.']).any(|segment| {
        let normalized = segment.to_ascii_lowercase();
        PLACEHOLDER_WORDS.contains(&normalized.as_str())
    })
}

fn contains_aws_access_key(content: &str) -> bool {
    content
        .split(|character: char| !character.is_ascii_alphanumeric())
        .any(|token| {
            token.len() == 20
                && (token.starts_with("AKIA") || token.starts_with("ASIA"))
                && token
                    .chars()
                    .all(|character| character.is_ascii_uppercase() || character.is_ascii_digit())
                && distinct_ascii_count(token) >= 8
                && !is_repeated_pattern(token)
                && !has_monotonic_sequence(token, 8)
        })
}

fn highest_secret_assignment_score(content: &str) -> Option<(usize, u8)> {
    let lines = content.lines().collect::<Vec<_>>();
    let mut line_index = 0usize;
    let mut highest = None;

    while line_index < lines.len() {
        let line = lines[line_index];
        let Some(separator_index) = find_assignment_separator(line) else {
            line_index += 1;
            continue;
        };

        let key_score = secret_key_score(&line[..separator_index]);
        if key_score == 0 {
            line_index += 1;
            continue;
        }

        let Some((value, final_line_index)) =
            extract_assignment_value(&lines, line_index, separator_index)
        else {
            line_index += 1;
            continue;
        };

        let score = score_secret_value(key_score, &value);
        if score > highest.map_or(0, |(_, current_score)| current_score) {
            highest = Some((line_index + 1, score));
        }

        line_index = final_line_index.saturating_add(1).max(line_index + 1);
    }

    highest.filter(|(_, score)| *score >= SECRET_ASSIGNMENT_THRESHOLD)
}

#[cfg(test)]
fn secret_assignment_score(content: &str) -> u8 {
    highest_secret_assignment_score(content).map_or(0, |(_, score)| score)
}

fn score_secret_value(key_score: u8, value: &str) -> u8 {
    let trimmed_value = value.trim();
    if is_placeholder_value(trimmed_value) {
        return 0;
    }

    let Some(features) = secret_value_features(trimmed_value) else {
        return 0;
    };
    if !(20..=2_048).contains(&features.effective_len) {
        return 0;
    }

    let length_score = match features.effective_len {
        64.. => 30,
        48..=63 => 26,
        32..=47 => 22,
        28..=31 => 17,
        _ => 12,
    };
    let class_score = match features.class_count {
        4 => 24,
        3 => 21,
        2 => 14,
        _ if features.effective_len >= 40 => 6,
        _ => 0,
    };
    let distinct_score = match features.distinct_count {
        24.. => 22,
        16..=23 => 18,
        10..=15 => 13,
        8..=9 => 8,
        _ => 0,
    };

    key_score
        .saturating_add(length_score)
        .saturating_add(class_score)
        .saturating_add(distinct_score)
        .min(100)
}

fn find_assignment_separator(line: &str) -> Option<usize> {
    let bytes = line.as_bytes();

    for (index, byte) in bytes.iter().enumerate() {
        if *byte != b'=' {
            continue;
        }

        let previous = index
            .checked_sub(1)
            .and_then(|position| bytes.get(position));
        let next = bytes.get(index + 1);
        if previous.is_some_and(|value| matches!(*value, b'=' | b'!' | b'<' | b'>'))
            || next.is_some_and(|value| matches!(*value, b'=' | b'>'))
        {
            continue;
        }

        return Some(index);
    }

    let separator_index = line.find(':')?;
    is_colon_assignment_key(&line[..separator_index]).then_some(separator_index)
}

fn is_colon_assignment_key(key_part: &str) -> bool {
    let mut candidate = key_part.trim();
    if let Some(rest) = candidate.strip_prefix("- ") {
        candidate = rest.trim_start();
    }
    candidate = candidate
        .trim_start_matches(|character| matches!(character, '{' | '[' | ','))
        .trim();

    let first = candidate.as_bytes().first().copied();
    let last = candidate.as_bytes().last().copied();
    let first_is_quote = first == Some(b'"') || first == Some(39);
    let last_is_quote = last == Some(b'"') || last == Some(39);
    if first_is_quote || last_is_quote {
        if first != last || candidate.len() < 2 {
            return false;
        }
        candidate = &candidate[1..candidate.len() - 1];
    }

    !candidate.is_empty()
        && candidate.len() <= 128
        && candidate.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.')
        })
}

fn secret_key_score(key_part: &str) -> u8 {
    key_part
        .split(|character: char| {
            !(character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
        })
        .filter(|candidate| !candidate.is_empty())
        .filter_map(score_secret_key_name)
        .max()
        .unwrap_or(0)
}

fn score_secret_key_name(candidate: &str) -> Option<u8> {
    let normalized = candidate.to_ascii_lowercase().replace('-', "_");
    if KEY_PLACEHOLDER_MARKERS
        .iter()
        .any(|marker| normalized.contains(marker))
    {
        return None;
    }

    if STRONG_SECRET_KEYS.contains(&normalized.as_str()) {
        return Some(40);
    }
    if MODERATE_SECRET_KEYS.contains(&normalized.as_str()) {
        return Some(32);
    }

    if STRONG_SECRET_KEYS
        .iter()
        .any(|key| normalized.ends_with(&format!("_{key}")))
    {
        return Some(37);
    }
    if MODERATE_SECRET_KEYS
        .iter()
        .any(|key| normalized.ends_with(&format!("_{key}")))
    {
        return Some(29);
    }

    None
}

fn extract_assignment_value(
    lines: &[&str],
    line_index: usize,
    separator_index: usize,
) -> Option<(String, usize)> {
    let value_text = lines
        .get(line_index)?
        .get(separator_index + 1..)?
        .trim_start();
    let first_character = value_text.chars().next()?;

    if matches!(first_character, '"' | '\'') {
        return extract_quoted_value(lines, line_index, value_text, first_character);
    }
    if is_yaml_block_marker(value_text) {
        return extract_yaml_block_value(lines, line_index, value_text);
    }

    let value = strip_unquoted_comment(value_text)
        .trim()
        .trim_end_matches([',', ';'])
        .trim()
        .to_owned();
    (!value.is_empty()).then_some((value, line_index))
}

fn extract_quoted_value(
    lines: &[&str],
    start_line_index: usize,
    value_text: &str,
    quote: char,
) -> Option<(String, usize)> {
    let mut value = String::new();
    let mut line_index = start_line_index;
    let mut fragment = value_text.get(quote.len_utf8()..)?;
    let mut consumed_lines = 1usize;

    loop {
        let mut characters = fragment.chars().peekable();
        let mut escaped = false;

        while let Some(character) = characters.next() {
            if quote == '\'' && character == '\'' {
                if characters.peek().is_some_and(|next| *next == '\'') {
                    value.push('\'');
                    characters.next();
                    continue;
                }
                return Some((value, line_index));
            }

            if quote == '"' && character == '"' && !escaped {
                return Some((value, line_index));
            }

            value.push(character);
            if quote == '"' && character == '\\' {
                escaped = !escaped;
            } else {
                escaped = false;
            }

            if value.len() > MAX_SECRET_VALUE_BYTES {
                return None;
            }
        }

        consumed_lines += 1;
        if consumed_lines > MAX_SECRET_VALUE_LINES {
            return None;
        }
        line_index += 1;
        fragment = *lines.get(line_index)?;
        value.push('\n');
    }
}

fn is_yaml_block_marker(value: &str) -> bool {
    let mut characters = value.chars();
    matches!(characters.next(), Some('|' | '>'))
        && characters.all(|character| matches!(character, '+' | '-' | '1'..='9'))
}

fn extract_yaml_block_value(
    lines: &[&str],
    start_line_index: usize,
    marker: &str,
) -> Option<(String, usize)> {
    let base_indent = indentation(lines.get(start_line_index)?);
    let folded = marker.starts_with('>');
    let mut value = String::new();
    let mut line_index = start_line_index + 1;
    let mut final_line_index = start_line_index;
    let mut consumed_lines = 0usize;

    while let Some(line) = lines.get(line_index) {
        if !line.trim().is_empty() && indentation(line) <= base_indent {
            break;
        }

        consumed_lines += 1;
        if consumed_lines > MAX_SECRET_VALUE_LINES {
            return None;
        }

        if !value.is_empty() {
            value.push(if folded { ' ' } else { '\n' });
        }
        value.push_str(line.trim());
        if value.len() > MAX_SECRET_VALUE_BYTES {
            return None;
        }

        final_line_index = line_index;
        line_index += 1;
    }

    (!value.trim().is_empty()).then_some((value, final_line_index))
}

fn strip_unquoted_comment(value: &str) -> &str {
    let bytes = value.as_bytes();
    let mut index = 0usize;

    while index < bytes.len() {
        if bytes[index] == b'#' && index > 0 && bytes[index - 1].is_ascii_whitespace() {
            return &value[..index];
        }
        if bytes[index] == b'/'
            && bytes.get(index + 1) == Some(&b'/')
            && index > 0
            && bytes[index - 1].is_ascii_whitespace()
        {
            return &value[..index];
        }
        index += 1;
    }

    value
}

fn indentation(line: &str) -> usize {
    line.bytes().take_while(u8::is_ascii_whitespace).count()
}

struct SecretValueFeatures {
    effective_len: usize,
    class_count: usize,
    distinct_count: usize,
}

fn secret_value_features(value: &str) -> Option<SecretValueFeatures> {
    let mut has_lowercase = false;
    let mut has_uppercase = false;
    let mut has_digit = false;
    let mut has_symbol = false;
    let mut seen = [false; 128];
    let mut effective_len = 0usize;

    for byte in value.bytes() {
        if byte.is_ascii_whitespace() {
            continue;
        }
        if !byte.is_ascii_graphic() {
            return None;
        }

        effective_len += 1;
        has_lowercase |= byte.is_ascii_lowercase();
        has_uppercase |= byte.is_ascii_uppercase();
        has_digit |= byte.is_ascii_digit();
        has_symbol |= !byte.is_ascii_alphanumeric();
        seen[usize::from(byte)] = true;
    }

    let class_count = [has_lowercase, has_uppercase, has_digit, has_symbol]
        .into_iter()
        .filter(|present| *present)
        .count();
    let distinct_count = seen.into_iter().filter(|present| *present).count();

    Some(SecretValueFeatures {
        effective_len,
        class_count,
        distinct_count,
    })
}

fn is_placeholder_value(value: &str) -> bool {
    let lower_value = value.trim().to_ascii_lowercase();
    if lower_value.is_empty()
        || lower_value.starts_with('$')
        || lower_value.starts_with("{{")
        || lower_value.starts_with('<')
        || lower_value.contains("${")
        || lower_value.contains("secrets.")
        || lower_value.contains("process.env")
        || lower_value.contains("std::env")
        || lower_value.contains("os.environ")
        || lower_value.contains("example.com")
        || lower_value.contains("localhost")
        || lower_value.contains("127.0.0.1")
        || lower_value.contains("username:password")
        || lower_value.contains("user:pass")
        || lower_value.contains("...")
        || matches!(
            lower_value.as_str(),
            "false" | "none" | "null" | "true" | "undefined"
        )
    {
        return true;
    }

    if VALUE_PLACEHOLDER_MARKERS
        .iter()
        .any(|marker| lower_value.contains(marker))
    {
        return true;
    }

    let compact = lower_value
        .bytes()
        .filter(|byte| !byte.is_ascii_whitespace())
        .collect::<Vec<_>>();

    compact.len() < 20
        || distinct_ascii_count(&lower_value) <= 3
        || is_repeated_pattern_bytes(&compact)
        || has_monotonic_sequence_bytes(&compact, 8)
        || looks_like_prose(&lower_value)
}

fn looks_like_prose(value: &str) -> bool {
    let word_count = value.split_ascii_whitespace().count();
    if word_count < 4 {
        return false;
    }

    let mut letters = 0usize;
    let mut digits = 0usize;
    let mut symbols = 0usize;
    for byte in value.bytes().filter(|byte| !byte.is_ascii_whitespace()) {
        if byte.is_ascii_alphabetic() {
            letters += 1;
        } else if byte.is_ascii_digit() {
            digits += 1;
        } else {
            symbols += 1;
        }
    }

    letters >= 24 && digits == 0 && symbols <= 2
}

fn distinct_ascii_count(value: &str) -> usize {
    let mut seen = [false; 128];
    for byte in value.bytes().filter(u8::is_ascii) {
        seen[usize::from(byte)] = true;
    }
    seen.into_iter().filter(|present| *present).count()
}

fn is_repeated_pattern(value: &str) -> bool {
    let compact = value
        .bytes()
        .filter(|byte| !byte.is_ascii_whitespace())
        .collect::<Vec<_>>();
    is_repeated_pattern_bytes(&compact)
}

fn is_repeated_pattern_bytes(value: &[u8]) -> bool {
    if value.len() < 20 {
        return false;
    }

    (1..=8).any(|period| {
        value.len().is_multiple_of(period)
            && value
                .iter()
                .enumerate()
                .all(|(index, byte)| *byte == value[index % period])
    })
}

fn has_monotonic_sequence(value: &str, required_run: usize) -> bool {
    let compact = value
        .bytes()
        .filter(u8::is_ascii_alphanumeric)
        .collect::<Vec<_>>();
    has_monotonic_sequence_bytes(&compact, required_run)
}

fn has_monotonic_sequence_bytes(value: &[u8], required_run: usize) -> bool {
    if value.len() < required_run {
        return false;
    }

    let mut ascending = 1usize;
    let mut descending = 1usize;
    for pair in value.windows(2) {
        ascending = if pair[1] == pair[0].wrapping_add(1) {
            ascending + 1
        } else {
            1
        };
        descending = if pair[0] == pair[1].wrapping_add(1) {
            descending + 1
        } else {
            1
        };
        if ascending >= required_run || descending >= required_run {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::{
        SECRET_ASSIGNMENT_THRESHOLD, contains_aws_access_key, contains_prefixed_token, corpus,
        secret_assignment_score, secret_signal_score,
    };

    #[test]
    fn detects_current_provider_token_shapes() {
        for probe in corpus::provider_positive_probes() {
            assert_eq!(secret_signal_score(&probe), 100, "missed provider probe");
        }
    }

    #[test]
    fn rejects_provider_documentation_and_placeholders() {
        for probe in corpus::provider_negative_probes() {
            assert_eq!(secret_signal_score(&probe), 0, "flagged provider fixture");
        }
    }

    #[test]
    fn detects_aws_access_key_shape() {
        let probe = ["AKIA", "Q7W9E2R4T6Y8U1I3"].concat();
        assert!(contains_aws_access_key(&probe));
    }

    #[test]
    fn handles_multiline_and_escaped_assignments() {
        for probe in corpus::multiline_positive_probes() {
            assert!(
                secret_assignment_score(&probe) >= SECRET_ASSIGNMENT_THRESHOLD,
                "missed multiline probe"
            );
        }
    }

    #[test]
    fn ignores_prefixed_placeholders() {
        assert!(!contains_prefixed_token(
            "sk-proj-example-placeholder-value-that-is-long",
            &["sk-proj-"],
            20
        ));
    }

    #[test]
    fn calibrated_corpus_meets_quality_floors() {
        let cases = corpus::assignment_cases();
        let case_names = cases.iter().map(|case| case.name).collect::<Vec<_>>();
        assert!(
            cases.len() >= 64,
            "calibration corpus is too small: {case_names:?}"
        );
        assert!(
            cases.iter().filter(|case| case.expected).count() >= 28,
            "calibration corpus needs more positive cases"
        );
        assert!(
            cases.iter().filter(|case| !case.expected).count() >= 32,
            "calibration corpus needs more hard negatives"
        );

        let scored = cases
            .iter()
            .map(|case| (secret_signal_score(&case.content), case.expected))
            .collect::<Vec<_>>();
        let metrics = calibration_metrics(&scored, SECRET_ASSIGNMENT_THRESHOLD);

        assert!(
            metrics.average_precision >= 0.98,
            "average precision below floor: {:.4}",
            metrics.average_precision
        );
        assert!(
            metrics.precision >= 0.95,
            "precision below floor: {:.4}",
            metrics.precision
        );
        assert!(
            metrics.recall >= 0.95,
            "recall below floor: {:.4}",
            metrics.recall
        );
        assert!(metrics.f1 >= 0.95, "F1 below floor: {:.4}", metrics.f1);
    }

    struct CalibrationMetrics {
        average_precision: f64,
        precision: f64,
        recall: f64,
        f1: f64,
    }

    fn calibration_metrics(scored_labels: &[(u8, bool)], threshold: u8) -> CalibrationMetrics {
        let true_positives = scored_labels
            .iter()
            .filter(|(score, expected)| *score >= threshold && *expected)
            .count();
        let false_positives = scored_labels
            .iter()
            .filter(|(score, expected)| *score >= threshold && !*expected)
            .count();
        let false_negatives = scored_labels
            .iter()
            .filter(|(score, expected)| *score < threshold && *expected)
            .count();

        let precision = ratio(true_positives, true_positives + false_positives);
        let recall = ratio(true_positives, true_positives + false_negatives);
        let f1 = if precision + recall == 0.0 {
            0.0
        } else {
            2.0 * precision * recall / (precision + recall)
        };

        CalibrationMetrics {
            average_precision: average_precision(scored_labels),
            precision,
            recall,
            f1,
        }
    }

    fn average_precision(scored_labels: &[(u8, bool)]) -> f64 {
        let positive_count = scored_labels
            .iter()
            .filter(|(_, expected)| *expected)
            .count();
        assert!(positive_count > 0);

        let mut thresholds = scored_labels
            .iter()
            .map(|(score, _)| *score)
            .collect::<Vec<_>>();
        thresholds.sort_unstable_by(|left, right| right.cmp(left));
        thresholds.dedup();

        let mut previous_recall = 0.0;
        let mut area = 0.0;
        for threshold in thresholds {
            let selected_count = scored_labels
                .iter()
                .filter(|(score, _)| *score >= threshold)
                .count();
            let true_positives = scored_labels
                .iter()
                .filter(|(score, expected)| *score >= threshold && *expected)
                .count();
            let precision = ratio(true_positives, selected_count);
            let recall = ratio(true_positives, positive_count);
            area += (recall - previous_recall) * precision;
            previous_recall = recall;
        }

        area
    }

    fn ratio(numerator: usize, denominator: usize) -> f64 {
        if denominator == 0 {
            return 0.0;
        }
        let numerator = u32::try_from(numerator).expect("calibration corpus fits in u32");
        let denominator = u32::try_from(denominator).expect("calibration corpus fits in u32");
        f64::from(numerator) / f64::from(denominator)
    }
}
