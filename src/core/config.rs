pub struct Config {
    pub name: String,
    pub tabs: Vec<String>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            name: "Command Manager".to_string(),
            tabs: vec![
                "Tab 1".to_string(),
                "Tab 2".to_string(),
                "Tab 3".to_string(),
            ],
        }
    }
}
