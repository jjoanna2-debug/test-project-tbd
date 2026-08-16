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
        ["Documentation mentions ", "ghp_", " without a complete token"].concat(),
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
    let values = (1..=28)
        .map(|seed| generated_secret(seed, 40 + seed % 17))
        .collect::<Vec<_>>();

    push(cases, "shell API key", quoted_assignment("SERVICE_API_KEY", &values[0]), true);
    push(cases, "JSON client secret", json_assignment("client_secret", &values[1]), true);
    push(cases, "Rust access token", typed_assignment("accessToken", &values[2]), true);
    push(cases, "YAML password with comment", unquoted_assignment("password", &values[3]), true);
    push(cases, "TOML private key", quoted_assignment("private_key", &values[4]), true);
    push(cases, "refresh token", quoted_assignment("refresh_token", &values[5]), true);
    push(cases, "webhook secret", quoted_assignment("webhook_secret", &values[6]), true);
    push(cases, "signing key", quoted_assignment("signing_key", &hex_secret(7, 64)), true);
    push(cases, "session token", quoted_assignment("session_token", &base64_secret(8, 56)), true);
    push(cases, "authentication token", quoted_assignment("auth_token", &values[9]), true);
    push(cases, "bearer token", quoted_assignment("bearer_token", &values[10]), true);
    push(cases, "passphrase", quoted_assignment("passphrase", &values[11]), true);
    push(cases, "master key", quoted_assignment("master_key", &values[12]), true);
    push(cases, "encryption key", quoted_assignment("encryption_key", &values[13]), true);
    push(cases, "database password", quoted_assignment("db_password", &values[14]), true);
    push(cases, "service account key", quoted_assignment("service_account_key", &values[15]), true);
    push(cases, "SAS token", quoted_assignment("sas_token", &values[16]), true);
    push(cases, "credentials", quoted_assignment("credentials", &values[17]), true);
    push(cases, "database URL", database_url_assignment(&values[18]), true);
    push(cases, "connection string", connection_string_assignment(&values[19]), true);
    push(cases, "authorization header", authorization_assignment(&values[20]), true);
    push(cases, "multiline quoted secret", multiline_quoted_assignment("client_secret", &values[21], &values[22]), true);
    push(cases, "YAML literal secret", yaml_block_assignment("private_key", '|', &values[23], &values[24]), true);
    push(cases, "YAML folded secret", yaml_block_assignment("webhook_secret", '>', &values[25], &values[26]), true);
    push(cases, "escaped quoted secret", escaped_quoted_assignment("api_key", &values[27]), true);
    push(cases, "commented leaked secret", ["# ", &quoted_assignment("api_key", &generated_secret(31, 44))].concat(), true);
    push(cases, "JSON trailing comma", [json_assignment("refresh_token", &generated_secret(32, 44)), ",".to_owned()].concat(), true);
    push(cases, "production-prefixed key", quoted_assignment("PROD_SERVICE_API_KEY", &generated_secret(33, 44)), true);
    push(cases, "uppercase access token", quoted_assignment("PAYMENTS_ACCESS_TOKEN", &generated_secret(34, 44)), true);
    push(cases, "long hexadecimal secret", quoted_assignment("secret_key", &hex_secret(35, 96)), true);
}

fn add_positive_provider_tokens(cases: &mut Vec<CalibrationCase>) {
    for (index, probe) in provider_positive_probes().into_iter().enumerate() {
        push(
            cases,
            provider_case_name(index),
            ["credential: ", &probe].concat(),
            true,
        );
    }

    push(
        cases,
        "AWS access key identifier",
        ["AWS_ACCESS_KEY_ID=", &["AKIA", "Q7W9E2R4T6Y8U1I3"].concat()].concat(),
        true,
    );
}

fn add_negative_placeholders(cases: &mut Vec<CalibrationCase>) {
    push(cases, "named API placeholder", quoted_assignment("api_key", "your_api_key_here"), false);
    push(cases, "environment expansion", quoted_assignment("client_secret", "${CLIENT_SECRET}"), false);
    push(cases, "template expansion", quoted_assignment("access_token", "{{ secrets.ACCESS_TOKEN }}"), false);
    push(cases, "process environment reference", quoted_assignment("auth_token", "process.env.AUTH_TOKEN"), false);
    push(cases, "Rust environment reference", quoted_assignment("password", "std::env::var(DATABASE_PASSWORD)"), false);
    push(cases, "Python environment reference", quoted_assignment("password", "os.environ[DATABASE_PASSWORD]"), false);
    push(cases, "redacted value", quoted_assignment("private_key", "redacted-redacted-redacted"), false);
    push(cases, "repeated x fixture", quoted_assignment("api_key", &"x".repeat(48)), false);
    push(cases, "repeated block fixture", quoted_assignment("api_key", &"A7z9".repeat(12)), false);
    push(cases, "ascending alphabet fixture", quoted_assignment("api_key", "abcdefghijklmnopqrstuvwxyz0123456789"), false);
    push(cases, "descending alphabet fixture", quoted_assignment("api_key", "zyxwvutsrqponmlkjihgfedcba9876543210"), false);
    push(cases, "example host database URL", quoted_assignment("database_url", "postgresql://user:secret-value@example.com/app"), false);
    push(cases, "localhost connection string", quoted_assignment("connection_string", "postgresql://username:password@localhost/app"), false);
    push(cases, "angle-bracket placeholder", quoted_assignment("webhook_secret", "<insert-webhook-secret-here>"), false);
    push(cases, "ellipsis placeholder", quoted_assignment("signing_key", &["sk_live_", "................................."].concat()), false);
    push(cases, "boolean secret field", unquoted_assignment("secret", "false"), false);
    push(cases, "null password field", unquoted_assignment("password", "null"), false);
    push(cases, "short password", quoted_assignment("password", "correct-horse"), false);
    push(cases, "prose password explanation", quoted_assignment("password", "this value is supplied by the deployment environment"), false);
    push(cases, "YAML prose block", yaml_block_assignment("secret", '|', "this value is documented for operators", "and is not stored in this repository"), false);
}

fn add_negative_key_semantics(cases: &mut Vec<CalibrationCase>) {
    let random = generated_secret(71, 48);
    push(cases, "token count", quoted_assignment("token_count", &random), false);
    push(cases, "secretary field", quoted_assignment("secretary", &random), false);
    push(cases, "password policy", quoted_assignment("password_policy", &random), false);
    push(cases, "API key name", quoted_assignment("api_key_name", &random), false);
    push(cases, "client secret field name", quoted_assignment("client_secret_field", &random), false);
    push(cases, "access token URL", quoted_assignment("access_token_url", &random), false);
    push(cases, "token endpoint", quoted_assignment("token_endpoint", &random), false);
    push(cases, "token type", quoted_assignment("token_type", &random), false);
    push(cases, "password reset URL", quoted_assignment("password_reset_url", &random), false);
    push(cases, "secret scan threshold", quoted_assignment("secret_scan_threshold", &random), false);
    push(cases, "private key path", quoted_assignment("private_key_path", "/var/run/keys/service.pem"), false);
    push(cases, "connection string format", quoted_assignment("connection_string_format", &random), false);
    push(cases, "example API key fixture", quoted_assignment("example_api_key", &random), false);
    push(cases, "test password fixture", quoted_assignment("test_password", &random), false);
    push(cases, "mock client secret fixture", quoted_assignment("mock_client_secret", &random), false);
    push(cases, "fixture token", quoted_assignment("fixture_token", &random), false);
    push(cases, "sample connection string", quoted_assignment("sample_connection_string", &random), false);
    push(cases, "public key material", quoted_assignment("public_key", &base64_secret(72, 80)), false);
    push(cases, "checksum", quoted_assignment("checksum", &hex_secret(73, 64)), false);
    push(cases, "request identifier", quoted_assignment("request_token_id", &random), false);
}

fn add_negative_documentation_examples(cases: &mut Vec<CalibrationCase>) {
    for (index, probe) in provider_negative_probes().into_iter().enumerate() {
        push(
            cases,
            provider_negative_case_name(index),
            ["documentation: ", &probe].concat(),
            false,
        );
    }

    push(cases, "schema declaration", "client_secret: { type: string, minLength: 32 }".to_owned(), false);
    push(cases, "JWT header constant", quoted_assignment("token_header", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"), false);
    push(cases, "authorization scheme", quoted_assignment("authorization_scheme", "Bearer"), false);
    push(cases, "CSRF field name", quoted_assignment("csrf_token_name", "csrfmiddlewaretoken"), false);
    push(cases, "expiration setting", unquoted_assignment("access_token_expires_in", "3600"), false);
    push(cases, "password reset TTL", unquoted_assignment("password_reset_token_ttl", "900"), false);
}

fn push(
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
    const ALPHABET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz23456789_-";
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
    const BASE64: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
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
