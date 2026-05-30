use minijinja::Environment;
use reqwest::Client;
use std::env;

pub struct AppState {
    pub templates: Environment<'static>,
    pub mealie_url: String,
    pub mealie_token: String,
    pub http: Client,
    pub content_dir: String,
    pub data_dir: String,
}

impl AppState {
    pub fn new() -> anyhow::Result<Self> {
        let templates_dir = env::var("TEMPLATES_DIR").unwrap_or_else(|_| "templates".into());
        let mut env = Environment::new();
        env.set_loader(minijinja::path_loader(templates_dir));

        Ok(Self {
            templates: env,
            mealie_url: env::var("MEALIE_URL").unwrap_or_else(|_| "http://mealie:9000".into()),
            mealie_token: env::var("MEALIE_TOKEN").unwrap_or_default(),
            http: Client::new(),
            content_dir: env::var("CONTENT_DIR").unwrap_or_else(|_| "content".into()),
            data_dir: env::var("DATA_DIR").unwrap_or_else(|_| "data".into()),
        })
    }
}
