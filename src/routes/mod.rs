pub mod blog;
pub mod notes;
pub mod recipes;
pub mod workouts;

use axum::extract::State;
use axum::response::Html;
use minijinja::context;
use std::{path::PathBuf, sync::Arc};

use crate::{content, error::Result, state::AppState};

pub async fn index(State(s): State<Arc<AppState>>) -> Result<Html<String>> {
    let tmpl = s.templates.get_template("index.html")?;
    Ok(Html(tmpl.render(context! {})?))
}

pub async fn bookshelf(State(s): State<Arc<AppState>>) -> Result<Html<String>> {
    let path = PathBuf::from(&s.content_dir).join("bookshelf.md");
    let page = content::parse_file(&path)?;
    let tmpl = s.templates.get_template("page.html")?;
    Ok(Html(tmpl.render(context! { page })?))
}
