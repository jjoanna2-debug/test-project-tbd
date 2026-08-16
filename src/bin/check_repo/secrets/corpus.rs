// Copyright 2026 Jean-Claude Joanna
// SPDX-License-Identifier: Apache-2.0

pub(super) struct CalibrationCase {
    pub(super) name: &'static str,
    pub(super) content: String,
    pub(super) expected: bool,
}

pub(super) fn assignment_cases() -> Vec<CalibrationCase> {
    let mut cases = Vec::new();

    add_positive_assignments(&mut cases);
    add_positive_provider_tokens(&mut cases);
    add_negative_placeholders(&mut cases);
    add_negative_key_semantics(&mut cases);
    add_negative_documentation_examples(&mut cases);

    cases
}

pub(super) fn provider_positive_probes() -> Vec<String> {
    vec![
        provider_probe("ghp_", 1, 36),
        provider_probe("github_pat_", 2, 48),
        github_stateless_probe(3),
        provider_probe("glpat-", 4, 32),
        provider_probe("gldt-", 5, 32),
        provider_probe("glrt-", 6, 32),
        provider_probe("glwt-", 7, 32),
        provider_probe("sk-proj-", 8, 48),
        provider_probe("sk-svcacct-", 9, 48),
        provider_probe("xoxb-", 10, 40),
        provider_probe("sk_live_", 11, 32),
        provider_probe("npm_", 12, 40),
        provider_probe("AIza", 13, 36),
        provider_probe("SG.", 14, 56),
    ]
}

pub(super) fn provider_negative_probes() -> Vec<String> {
    vec![
        ["ghp_", "example_placeholder_value_that_is_long_enough"].concat(),
        ["github_pat_", "replace_me_with_a_real_token_value"].concat(),
        ["ghs_", "APPID_JWT"].concat(),
        ["glpat-", "example-placeholder-value-that-is-long"].concat(),
        ["glwt-", "sample-token-value-that-is-long-enough"].concat(),
        ["sk-proj-", "your-api-key-goes-here-not-a-real-key"].concat(),
        ["xoxb-", "redacted-redacted-redacted-redacted"].concat(),
        ["sk_live_", "12345678901234567890123456789012"].concat(),
        ["npm_", &"x".repeat(40)].concat(),
        ["AIza", "abcdefghijklmnopqrstuvwxyz0123456789"].concat(),
        ["SG.", &"ABCD".repeat(14)].concat(),
        [
            "Documentation mentions ",
            "ghp_",
            " without a complete token",
        ]
        .concat(),
    ]
}

pub(super) fn multiline_positive_probes() -> Vec<String> {
    let first = generated_secret(61, 24);
    let second = generated_secret(62, 24);

    vec![
        multiline_quoted_assignment("client_secret", &first, &second),
        yaml_block_assignment("signing_key", '|', &first, &second),
        yaml_block_assignment("webhook_secret", '>', &first, &second),
        escaped_quoted_assignment("api_key", &generated_secret(63, 38)),
    ]
}

fn add_positive_assignments(cases: &mut Vec<CalibrationCase>) {
    const QUOTED_KEYS: &[(&str, &str)] = &[
        ("shell API key", "SERVICE_API_KEY"),
        ("TOML private key", "private_key"),
        ("refresh token", "refresh_token"),
        ("webhook secret", "webhook_secret"),
        ("authentication token", "auth_token"),
        ("bearer token", "bearer_token"),
        ("passphrase", "passphrase"),
        ("master key", "master_key"),
        ("encryption key", "encryption_key"),
        ("database password", "db_password"),
        ("service account key", "service_account_key"),
        ("SAS token", "sas_token"),
        ("credentials", "credentials"),
        ("session token", "session_token"),
        ("production-prefixed key", "PROD_SERVICE_API_KEY"),
        ("uppercase access token", "PAYMENTS_ACCESS_TOKEN"),
        ("session secret", "session_secret"),
        ("database password alias", "database_password"),
    ];

    for (index, &(name, key)) in QUOTED_KEYS.iter().enumerate() {
        let value = generated_secret(index + 1, 40 + index % 17);
        add_case(cases, name, quoted_assignment(key, &value), true);
    }

    let values = (31..=46)
        .map(|seed| generated_secret(seed, 44 + seed % 13))
        .collect::<Vec<_>>();

    add_case(
        cases,
        "JSON client secret",
        json_assignment("client_secret", &values[0]),
        true,
    );
    add_case(
        cases,
        "Rust access token",
        typed_assignment("accessToken", &values[1]),
        true,
    );
    add_case(
        cases,
        "YAML password with comment",
        unquoted_assignment("password", &values[2]),
        true,
    );
    add_case(
        cases,
        "signing key",
        quoted_assignment("signing_key", &hex_secret(34, 64)),
        true,
    );
    add_case(
        cases,
        "database URL",
        database_url_assignment(&values[4]),
        true,
    );
    add_case(
        cases,
        "connection string",
        connection_string_assignment(&values[5]),
        true,
    );
    add_case(
        cases,
        "authorization header",
        authorization_assignment(&values[6]),
        true,
    );
    add_case(
        cases,
        "multiline quoted secret",
        multiline_quoted_assignment("client_secret", &values[7], &values[8]),
        true,
    );
    add_case(
        cases,
        "YAML literal secret",
        yaml_block_assignment("private_key", '|', &values[9], &values[10]),
        true,
    );
    add_case(
        cases,
        "YAML folded secret",
        yaml_block_assignment("webhook_secret", '>', &values[11], &values[12]),
        true,
    );
    add_case(
        cases,
        "escaped quoted secret",
        escaped_quoted_assignment("api_key", &values[13]),
        true,
    );
    add_case(
        cases,
        "commented leaked secret",
        ["# ", &quoted_assignment("api_key", &values[14])].concat(),
        true,
    );
    add_case(
        cases,
        "JSON trailing comma",
        [json_assignment("refresh_token", &values[15]), ",".to_owned()].concat(),
        true,
    );
    add_case(
        cases,
        "long hexadecimal secret",
        quoted_assignment("secret_key", &hex_secret(47, 96)),
        true,
    );
}

fn add_positive_provider_tokens(cases: &mut Vec<CalibrationCase>) {
    for (index, probe) in provider_positive_probes().into_iter().enumerate() {
        add_case(
            cases,
            provider_case_name(index),
            ["credential: ", &probe].concat(),
            true,
        );
    }

    let aws_access_key = ["AKIA", "Q7W9E2R4T6Y8U1I3"].concat();
    add_case(
        cases,
        "AWS access key identifier",
        ["AWS_ACCESS_KEY_ID=", &aws_access_key].concat(),
        true,
    );
}

fn add_negative_placeholders(cases: &mut Vec<CalibrationCase>) {
    const STATIC_CASES: &[(&str, &str, &str)] = &[
        ("named API placeholder", "api_key", "your_api_key_here"),
        ("environment expansion", "client_secret", "${CLIENT_SECRET}"),
        (
            "template expansion",
            "access_token",
            "{{ secrets.ACCESS_TOKEN }}",
        ),
        (
            "process environment reference",
            "auth_token",
            "process.env.AUTH_TOKEN",
        ),
        (
            "Rust environment reference",
            "password",
            "std::env::var(DATABASE_PASSWORD)",
        ),
        (
            "Python environment reference",
            "password",
            "os.environ[DATABASE_PASSWORD]",
        ),
        (
            "redacted value",
            "private_key",
            "redacted-redacted-redacted",
        ),
        (
            "ascending alphabet fixture",
            "api_key",
            "abcdefghijklmnopqrstuvwxyz0123456789",
        ),
        (
            "descending alphabet fixture",
            "api_key",
            "zyxwvutsrqponmlkjihgfedcba9876543210",
        ),
        (
            "example host database URL",
            "database_url",
            "postgresql://user:secret-value@example.com/app",
        ),
        (
            "localhost connection string",
            "connection_string",
            "postgresql://username:password@localhost/app",
        ),
        (
            "angle-bracket placeholder",
            "webhook_secret",
            "<insert-webhook-secret-here>",
        ),
        (
            "prose password explanation",
            "password",
            "this value is supplied by the deployment environment",
        ),
    ];

    for &(name, key, value) in STATIC_CASES {
        add_case(cases, name, quoted_assignment(key, value), false);
    }

    add_case(
        cases,
        "repeated x fixture",
        quoted_assignment("api_key", &"x".repeat(48)),
        false,
    );
    add_case(
        cases,
        "repeated block fixture",
        quoted_assignment("api_key", &"A7z9".repeat(12)),
        false,
    );
    add_case(
        cases,
        "ellipsis placeholder",
        quoted_assignment(
            "signing_key",
            &["sk_live_", "................................."].concat(),
        ),
        false,
    );
    add_case(
        cases,
        "boolean secret field",
        unquoted_assignment("secret", "false"),
        false,
    );
    add_case(
        cases,
        "null password field",
        unquoted_assignment("password", "null"),
        false,
    );
    add_case(
        cases,
        "short password",
        quoted_assignment("password", "correct-horse"),
        false,
    );
    add_case(
        cases,
        "YAML prose block",
        yaml_block_assignment(
            "secret",
            '|',
            "this value is documented for operators",
            "and is not stored in this repository",
        ),
        false,
    );
}

fn add_negative_key_semantics(cases: &mut Vec<CalibrationCase>) {
    const KEYS: &[(&str, &str)] = &[
        ("token count", "token_count"),
        ("secretary field", "secretary"),
        ("password policy", "password_policy"),
        ("API key name", "api_key_name"),
        ("client secret field name", "client_secret_field"),
        ("access token URL", "access_token_url"),
        ("token endpoint", "token_endpoint"),
        ("token type", "token_type"),
        ("password reset URL", "password_reset_url"),
        ("secret scan threshold", "secret_scan_threshold"),
        ("connection string format", "connection_string_format"),
        ("example API key fixture", "example_api_key"),
        ("test password fixture", "test_password"),
        ("mock client secret fixture", "mock_client_secret"),
        ("fixture token", "fixture_token"),
        ("sample connection string", "sample_connection_string"),
        ("request identifier", "request_token_id"),
    ];

    let random = generated_secret(71, 48);
    for &(name, key) in KEYS {
        add_case(cases, name, quoted_assignment(key, &random), false);
    }

    add_case(
        cases,
        "private key path",
        quoted_assignment("private_key_path", "/var/run/keys/service.pem"),
        false,
    );
    add_case(
        cases,
        "public key material",
        quoted_assignment("public_key", &base64_secret(72, 80)),
        false,
    );
    add_case(
        cases,
        "checksum",
        quoted_assignment("checksum", &hex_secret(73, 64)),
        false,
    );
}

fn add_negative_documentation_examples(cases: &mut Vec<CalibrationCase>) {
    for (index, probe) in provider_negative_probes().into_iter().enumerate() {
        add_case(
            cases,
            provider_negative_case_name(index),
            ["documentation: ", &probe].concat(),
            false,
        );
    }

    add_case(
        cases,
        "schema declaration",
        "client_secret: { type: string, minLength: 32 }".to_owned(),
        false,
    );
    add_case(
        cases,
        "JWT header constant",
        quoted_assignment(
            "token_header",
            "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
        ),
        false,
    );
    add_case(
        cases,
        "authorization scheme",
        quoted_assignment("authorization_scheme", "Bearer"),
        false,
    );
    add_case(
        cases,
        "CSRF field name",
        quoted_assignment("csrf_token_name", "csrfmiddlewaretoken"),
        false,
    );
    add_case(
        cases,
        "expiration setting",
        unquoted_assignment("access_token_expires_in", "3600"),
        false,
    );
    add_case(
        cases,
        "password reset TTL",
        unquoted_assignment("password_reset_token_ttl", "900"),
        false,
    );
}

fn add_case(
    cases: &mut Vec<CalibrationCase>,
    name: &'static str,
    content: String,
    expected: bool,
) {
    cases.push(CalibrationCase {
        name,
        content,
        expected,
    });
}

fn quoted_assignment(key: &str, value: &str) -> String {
    [key, " = \"", value, "\""].concat()
}

fn unquoted_assignment(key: &str, value: &str) -> String {
    [key, " = ", value, " # deployment value"].concat()
}

fn json_assignment(key: &str, value: &str) -> String {
    ["{\"", key, "\": \"", value, "\"}"].concat()
}

fn typed_assignment(key: &str, value: &str) -> String {
    ["let ", key, ": &str = \"", value, "\";"].concat()
}

fn multiline_quoted_assignment(key: &str, first: &str, second: &str) -> String {
    [key, " = \"", first, "\n", second, "\""].concat()
}

fn escaped_quoted_assignment(key: &str, value: &str) -> String {
    [key, " = \"", value, "\\\"segment\""].concat()
}

fn yaml_block_assignment(key: &str, marker: char, first: &str, second: &str) -> String {
    [
        key,
        ": ",
        &marker.to_string(),
        "\n  ",
        first,
        "\n  ",
        second,
    ]
    .concat()
}

fn database_url_assignment(secret: &str) -> String {
    quoted_assignment(
        "database_url",
        &["postgresql://service:", secret, "@db.internal/prod"].concat(),
    )
}

fn connection_string_assignment(secret: &str) -> String {
    quoted_assignment(
        "connection_string",
        &["Server=db.internal;User=service;Password=", secret].concat(),
    )
}

fn authorization_assignment(secret: &str) -> String {
    quoted_assignment("authorization", &["Bearer ", secret].concat())
}

fn provider_probe(prefix: &str, seed: usize, tail_len: usize) -> String {
    [prefix, &generated_secret(seed, tail_len)].concat()
}

fn github_stateless_probe(seed: usize) -> String {
    [
        "ghs_18472_",
        &generated_secret(seed, 32),
        ".",
        &generated_secret(seed + 1, 32),
    ]
    .concat()
}

fn generated_secret(seed: usize, length: usize) -> String {
    const ALPHABET: &[u8] =
        b"ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz23456789_-";

    (0..length)
        .map(|index| {
            let position = (index * 17 + seed * 11 + index * index) % ALPHABET.len();
            char::from(ALPHABET[position])
        })
        .collect()
}

fn hex_secret(seed: usize, length: usize) -> String {
    const HEX: &[u8] = b"0123456789abcdef";

    (0..length)
        .map(|index| char::from(HEX[(index * 7 + seed * 5 + index / 3) % HEX.len()]))
        .collect()
}

fn base64_secret(seed: usize, length: usize) -> String {
    const BASE64: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    (0..length)
        .map(|index| {
            let position = (index * 19 + seed * 13 + index / 2) % BASE64.len();
            char::from(BASE64[position])
        })
        .collect()
}

fn provider_case_name(index: usize) -> &'static str {
    const NAMES: &[&str] = &[
        "GitHub classic token",
        "GitHub fine-grained token",
        "GitHub stateless installation token",
        "GitLab personal token",
        "GitLab deploy token",
        "GitLab runner token",
        "GitLab workspace token",
        "OpenAI project key",
        "OpenAI service account key",
        "Slack bot token",
        "Stripe live key",
        "npm token",
        "Google API key",
        "SendGrid key",
    ];

    NAMES[index]
}

fn provider_negative_case_name(index: usize) -> &'static str {
    const NAMES: &[&str] = &[
        "GitHub placeholder",
        "GitHub fine-grained placeholder",
        "GitHub format documentation",
        "GitLab placeholder",
        "GitLab workspace placeholder",
        "OpenAI placeholder",
        "Slack redacted token",
        "Stripe sequential fixture",
        "npm repeated fixture",
        "Google sequential fixture",
        "SendGrid repeated fixture",
        "GitHub prefix prose",
    ];

    NAMES[index]
}
