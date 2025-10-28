pub mod env {
    use dotenvy::dotenv;
    use std::collections::HashMap;
    use std::env;

    #[derive(Debug, Clone)]
    pub struct Env {
        vars: HashMap<String, String>,
    }

    impl Env {
        pub fn init() -> Self {
            dotenv().ok();
            let vars = env::vars().collect();
            Self { vars }
        }

        pub fn get(&self, key: &str) -> Option<&str> {
            self.vars.get(key).map(|s| s.as_str())
        }

        pub fn get_or(&self, key: &str, default: &str) -> String {
            self.get(key).unwrap_or(default).to_string()
        }

        pub fn get_parsed<T: std::str::FromStr>(&self, key: &str) -> Option<T> {
            self.get(key)?.parse().ok()
        }
    }
}
