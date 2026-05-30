use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Deserialize, Serialize)]
pub struct FrontMatter {
    pub title: String,
    pub date: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct Page {
    pub meta: FrontMatter,
    pub html: String,
    pub slug: String,
}

pub fn parse_file(path: &Path) -> anyhow::Result<Page> {
    let raw = fs::read_to_string(path)?;
    let (meta, body) = split_frontmatter(&raw)?;
    let html = render_markdown(body);
    let slug = path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    Ok(Page { meta, html, slug })
}

pub fn load_dir(dir: &Path) -> anyhow::Result<Vec<Page>> {
    let mut pages = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().is_some_and(|e| e == "md") {
            let stem = path.file_stem().unwrap_or_default().to_string_lossy();
            if stem == "_index" {
                continue;
            }
            match parse_file(&path) {
                Ok(p) => pages.push(p),
                Err(e) => tracing::warn!("skipping {}: {e}", path.display()),
            }
        }
    }
    pages.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));
    Ok(pages)
}

fn split_frontmatter(src: &str) -> anyhow::Result<(FrontMatter, &str)> {
    let src = src.trim_start_matches('\n');
    anyhow::ensure!(src.starts_with("---"), "no frontmatter");
    let rest = &src[3..].trim_start_matches('\n');
    let end = rest
        .find("\n---")
        .ok_or_else(|| anyhow::anyhow!("unclosed frontmatter"))?;
    let yaml = &rest[..end];
    let body = rest[end + 4..].trim_start_matches('\n');
    let meta: FrontMatter = serde_yaml::from_str(yaml)?;
    Ok((meta, body))
}

pub fn render_markdown(src: &str) -> String {
    let opts = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS;
    let parser = Parser::new_ext(src, opts);
    let mut out = String::new();
    html::push_html(&mut out, parser);
    out
}
