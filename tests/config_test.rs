#[cfg(test)]
mod tests {
    use crate::app::config::Config;

    #[test]
    fn test_config() {
        Config.from()
    }
}