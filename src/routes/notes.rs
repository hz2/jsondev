use axum::extract::{Path, State};
use axum::response::Html;
use minijinja::context;
use std::{path::PathBuf, sync::Arc};

use crate::{content, error::Result, state::AppState};

pub async fn list(State(s): State<Arc<AppState>>) -> Result<Html<String>> {
    let dir = PathBuf::from(&s.content_dir).join("notes");
    let pages = content::load_dir(&dir)?;
    let tmpl = s.templates.get_template("notes/list.html")?;
    Ok(Html(tmpl.render(context! { pages })?))
}

pub async fn post(
    State(s): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<Html<String>> {
    let path = PathBuf::from(&s.content_dir).join("notes").join(format!("{slug}.md"));
    let page = content::parse_file(&path)?;
    let tmpl = s.templates.get_template("notes/post.html")?;
    Ok(Html(tmpl.render(context! { page })?))
}
