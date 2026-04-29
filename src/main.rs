fn main() {
    println!("{}", status_message());
}

fn status_message() -> &'static str {
    "GitHub staging lab ready."
}

#[cfg(test)]
mod tests {
    use super::status_message;

    #[test]
    fn status_message_is_clear() {
        assert_eq!(status_message(), "GitHub staging lab ready.");
    }
}
