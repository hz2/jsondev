use axum::extract::{Path, State};
use axum::response::Html;
use minijinja::context;
use std::{path::PathBuf, sync::Arc};

use crate::{content, error::Result, state::AppState};

pub async fn list(State(s): State<Arc<AppState>>) -> Result<Html<String>> {
    let dir = PathBuf::from(&s.content_dir).join("blog");
    let pages = content::load_dir(&dir)?;
    let posts: Vec<_> = pages
        .iter()
        .map(|p| {
            serde_json::json!({
                "title": p.meta.title,
                "date": p.meta.date,
                "description": p.meta.description,
                "href": blog_href(p.meta.date.as_deref(), &p.slug),
            })
        })
        .collect();
    let tmpl = s.templates.get_template("blog/list.html")?;
    Ok(Html(tmpl.render(context! { posts })?))
}

pub async fn post(
    State(s): State<Arc<AppState>>,
    Path((_year, _month, slug)): Path<(String, String, String)>,
) -> Result<Html<String>> {
    let path = PathBuf::from(&s.content_dir)
        .join("blog")
        .join(format!("{slug}.md"));
    let page = content::parse_file(&path)?;
    let tmpl = s.templates.get_template("blog/post.html")?;
    Ok(Html(tmpl.render(context! { page })?))
}

fn blog_href(date: Option<&str>, slug: &str) -> String {
    if let Some(d) = date {
        let parts: Vec<&str> = d.splitn(3, '-').collect();
        if parts.len() >= 2 {
            return format!("/blog/{}/{}/{slug}", parts[0], parts[1]);
        }
    }
    format!("/blog/0000/00/{slug}")
}
