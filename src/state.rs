use minijinja::Environment;
use std::env;

pub struct AppState {
    pub templates: Environment<'static>,
    pub content_dir: String,
    pub data_dir: String,
    pub static_dir: String,
}

impl AppState {
    pub fn new() -> anyhow::Result<Self> {
        let templates_dir = env::var("TEMPLATES_DIR").unwrap_or_else(|_| "templates".into());
        let mut env = Environment::new();
        env.set_loader(minijinja::path_loader(templates_dir));

        Ok(Self {
            templates: env,
            content_dir: env::var("CONTENT_DIR").unwrap_or_else(|_| "content".into()),
            data_dir: env::var("DATA_DIR").unwrap_or_else(|_| "data".into()),
            static_dir: env::var("STATIC_DIR").unwrap_or_else(|_| "static".into()),
        })
    }
}
