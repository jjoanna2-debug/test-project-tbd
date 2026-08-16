// Copyright 2026 Jean-Claude Joanna
// SPDX-License-Identifier: Apache-2.0

const PREFIXED_SECRET_RULES: &[(&str, &[&str], usize)] = &[
    (
        "GitHub token",
        &["ghp_", "gho_", "ghu_", "ghs_", "ghr_"],
        30,
    ),
    ("GitHub fine-grained token", &["github_pat_"], 20),
    ("GitLab personal access token", &["glpat-"], 20),
    ("OpenAI API key", &["sk-proj-", "sk-svcacct-"], 20),
    ("Slack token", &["xoxb-", "xoxp-", "xoxa-", "xoxr-", "xoxs-"], 20),
    ("Stripe live secret key", &["sk_live_", "rk_live_"], 16),
    ("npm access token", &["npm_"], 30),
    ("Google API key", &["AIza"], 30),
    ("SendGrid API key", &["SG."], 40),
];

const SECRET_KEY_SUFFIXES: &[&str] = &[
    "access-token",
    "access_token",
    "accesstoken",
    "api-key",
    "api_key",
    "apikey",
    "auth-token",
    "auth_token",
    "authtoken",
    "bearer-token",
    "bearer_token",
    "bearertoken",
    "client-secret",
    "client_secret",
    "clientsecret",
    "password",
    "passwd",
    "private-key",
    "private_key",
    "privatekey",
    "pwd",
    "refresh-token",
    "refresh_token",
    "refreshtoken",
    "secret",
    "secret-key",
    "secret_key",
    "secretkey",
    "signing-key",
    "signing_key",
    "signingkey",
    "token",
    "webhook-secret",
    "webhook_secret",
    "webhooksecret",
];

const PLACEHOLDER_MARKERS: &[&str] = &[
    "changeme",
    "dummy-secret",
    "dummy_secret",
    "example",
    "not-a-real",
    "not_real",
    "placeholder",
    "redacted",
    "replace-me",
    "replace_me",
    "sample-secret",
    "sample_secret",
    "your-api",
    "your-secret",
    "your-token",
    "your_api",
    "your_secret",
    "your_token",
];

const PRIVATE_KEY_PREFIX: &str = "-----BEGIN ";
const PRIVATE_KEY_WORDS: [&str; 2] = ["PRIVATE ", "KEY-----"];
const SECRET_ASSIGNMENT_THRESHOLD: u8 = 65;

pub(crate) fn check_secret_content(
    relative_path: &str,
    content: &str,
    failures: &mut Vec<String>,
) {
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

fn contains_prefixed_token(content: &str, prefixes: &[&str], minimum_tail_len: usize) -> bool {
    content
        .split(|character: char| {
            !(character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.'))
        })
        .any(|token| {
            prefixes.iter().any(|prefix| {
                token.starts_with(prefix)
                    && token.len() >= prefix.len() + minimum_tail_len
                    && !is_placeholder_value(token)
            })
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
        })
}

fn highest_secret_assignment_score(content: &str) -> Option<(usize, u8)> {
    content
        .lines()
        .enumerate()
        .filter_map(|(line_index, line)| {
            let score = secret_assignment_score(line);
            (score >= SECRET_ASSIGNMENT_THRESHOLD).then_some((line_index + 1, score))
        })
        .max_by_key(|&(_, score)| score)
}

fn secret_assignment_score(line: &str) -> u8 {
    let Some(separator_index) = find_assignment_separator(line) else {
        return 0;
    };
    let key_score = secret_key_score(&line[..separator_index]);
    if key_score == 0 {
        return 0;
    }

    let Some(value) = extract_assignment_value(line, separator_index) else {
        return 0;
    };
    if !(20..=512).contains(&value.len()) || is_placeholder_value(value) {
        return 0;
    }

    let Some(features) = secret_value_features(value) else {
        return 0;
    };

    let length_score = match value.len() {
        64.. => 30,
        48..=63 => 25,
        32..=47 => 20,
        _ => 15,
    };
    let class_score = match features.class_count {
        4 => 25,
        3 => 22,
        2 => 15,
        _ if value.len() >= 32 => 5,
        _ => 0,
    };
    let distinct_score = match features.distinct_count {
        16.. => 20,
        10..=15 => 15,
        6..=9 => 10,
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

        let previous = index.checked_sub(1).and_then(|position| bytes.get(position));
        let next = bytes.get(index + 1);
        if previous.is_some_and(|value| matches!(*value, b'=' | b'!' | b'<' | b'>'))
            || next.is_some_and(|value| matches!(*value, b'=' | b'>'))
        {
            continue;
        }

        return Some(index);
    }

    line.find(':')
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
    let lower_candidate = candidate.to_ascii_lowercase();
    SECRET_KEY_SUFFIXES.iter().find_map(|suffix| {
        if lower_candidate == *suffix {
            return Some(35);
        }

        let prefix = lower_candidate.strip_suffix(*suffix)?;
        if prefix.ends_with('_') || prefix.ends_with('-') {
            Some(32)
        } else if !prefix.is_empty() {
            Some(24)
        } else {
            None
        }
    })
}

fn extract_assignment_value(line: &str, separator_index: usize) -> Option<&str> {
    let value_text = line.get(separator_index + 1..)?.trim_start();
    let first_character = value_text.chars().next()?;

    if matches!(first_character, '"' | '\'') {
        let rest = &value_text[first_character.len_utf8()..];
        let mut escaped = false;

        for (index, character) in rest.char_indices() {
            if character == first_character && !escaped {
                return Some(&rest[..index]);
            }

            if character == '\\' {
                escaped = !escaped;
            } else {
                escaped = false;
            }
        }

        return None;
    }

    let end_index = value_text
        .char_indices()
        .find(|(_, character)| {
            character.is_ascii_whitespace() || matches!(character, ',' | ';' | '#')
        })
        .map_or(value_text.len(), |(index, _)| index);

    Some(&value_text[..end_index])
}

struct SecretValueFeatures {
    class_count: usize,
    distinct_count: usize,
}

fn secret_value_features(value: &str) -> Option<SecretValueFeatures> {
    let mut has_lowercase = false;
    let mut has_uppercase = false;
    let mut has_digit = false;
    let mut has_symbol = false;
    let mut seen = [false; 128];

    for byte in value.bytes() {
        if !(byte.is_ascii_alphanumeric() || b"_./+=:@-".contains(&byte)) {
            return None;
        }

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
    {
        return true;
    }

    if PLACEHOLDER_MARKERS
        .iter()
        .any(|marker| lower_value.contains(marker))
    {
        return true;
    }

    let mut seen = [false; 128];
    for byte in lower_value.bytes() {
        if byte.is_ascii() {
            seen[usize::from(byte)] = true;
        }
    }

    seen.into_iter().filter(|present| *present).count() <= 3
}

#[cfg(test)]
mod tests {
    use super::{
        check_secret_content, contains_aws_access_key, contains_prefixed_token,
        secret_assignment_score,
    };

    #[test]
    fn detects_prefixed_provider_token_shapes() {
        let probes = [
            format!("ghp_{}", "A7z9".repeat(9)),
            format!("glpat-{}", "A7z9".repeat(6)),
            format!("sk-proj-{}", "A7z9".repeat(6)),
            format!("xoxb-{}", "A7z9".repeat(6)),
            format!("sk_live_{}", "A7z9".repeat(5)),
            format!("npm_{}", "A7z9".repeat(8)),
            format!("AIza{}", "A7z9".repeat(8)),
            format!("SG.{}", "A7z9".repeat(11)),
        ];

        for probe in probes {
            let mut failures = Vec::new();
            check_secret_content("probe.txt", &probe, &mut failures);
            assert!(!failures.is_empty(), "missed provider probe: {probe}");
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
    fn detects_aws_access_key_shape() {
        let probe = ["AKIA", "ABCDEFGHIJKLMNOP"].concat();
        assert!(contains_aws_access_key(&probe));
    }

    #[test]
    fn keeps_secret_assignment_average_precision_above_floor() {
        let generated = "A7z9_Qp2".repeat(4);
        let corpus = [
            (format!("export SERVICE_API_KEY={generated}"), true),
            (format!("\"client_secret\": \"{generated}\","), true),
            (format!("let accessToken: &str = \"{generated}\";"), true),
            (format!("password: {generated} # deployment value"), true),
            ("API_KEY=your_api_key_here".to_owned(), false),
            ("client_secret = \"${CLIENT_SECRET}\"".to_owned(), false),
            ("token_count = \"A7z9_Qp2A7z9_Qp2A7z9_Qp2\"".to_owned(), false),
            ("secretary = \"A7z9_Qp2A7z9_Qp2A7z9_Qp2\"".to_owned(), false),
            ("password = \"redacted-redacted-redacted\"".to_owned(), false),
            ("api_key = \"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\"".to_owned(), false),
        ];
        let scored = corpus
            .iter()
            .map(|(probe, positive)| (secret_assignment_score(probe), *positive))
            .collect::<Vec<_>>();

        assert!(average_precision(&scored) >= 0.95);
    }

    fn average_precision(scored_labels: &[(u8, bool)]) -> f64 {
        let positive_count = scored_labels.iter().filter(|(_, positive)| *positive).count();
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
                .filter(|(score, positive)| *score >= threshold && *positive)
                .count();
            let precision = ratio(true_positives, selected_count);
            let recall = ratio(true_positives, positive_count);
            area += (recall - previous_recall) * precision;
            previous_recall = recall;
        }

        area
    }

    fn ratio(numerator: usize, denominator: usize) -> f64 {
        let numerator = u32::try_from(numerator).expect("test corpus fits in u32");
        let denominator = u32::try_from(denominator).expect("test corpus fits in u32");
        f64::from(numerator) / f64::from(denominator)
    }
}
