use axum::extract::{Path, State};
use axum::response::Html;
use minijinja::context;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{error::Result, state::AppState};

#[derive(Deserialize, Serialize)]
pub struct RecipeSummary {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub image: Option<String>,
}

#[derive(Deserialize)]
struct RecipeListResponse {
    items: Vec<RecipeSummary>,
}

#[derive(Deserialize, Serialize)]
pub struct RecipeDetail {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub image: Option<String>,
    #[serde(rename = "recipeIngredient")]
    pub ingredients: Option<Vec<serde_json::Value>>,
    #[serde(rename = "recipeInstructions")]
    pub instructions: Option<Vec<serde_json::Value>>,
}

pub async fn list(State(s): State<Arc<AppState>>) -> Result<Html<String>> {
    let resp = s
        .http
        .get(format!("{}/api/recipes", s.mealie_url))
        .bearer_auth(&s.mealie_token)
        .query(&[("perPage", "100"), ("orderBy", "name"), ("orderDirection", "asc")])
        .send()
        .await?
        .json::<RecipeListResponse>()
        .await?;
    let tmpl = s.templates.get_template("recipes/list.html")?;
    Ok(Html(tmpl.render(context! { recipes => resp.items })?))
}

pub async fn detail(
    State(s): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<Html<String>> {
    let recipe = s
        .http
        .get(format!("{}/api/recipes/{slug}", s.mealie_url))
        .bearer_auth(&s.mealie_token)
        .send()
        .await?
        .json::<RecipeDetail>()
        .await?;
    let tmpl = s.templates.get_template("recipes/detail.html")?;
    Ok(Html(tmpl.render(context! { recipe })?))
}
