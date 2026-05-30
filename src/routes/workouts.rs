use axum::extract::State;
use axum::response::Html;
use minijinja::context;
use std::{fs, path::PathBuf, sync::Arc};

use crate::{error::Result, state::AppState};

pub async fn page(State(s): State<Arc<AppState>>) -> Result<Html<String>> {
    let dir = PathBuf::from(&s.data_dir);
    let running_stats = read_json(&dir.join("running_stats.json"))?;
    let recent_runs = read_json(&dir.join("recent_runs.json"))?;
    let weekly_mileage = read_json(&dir.join("weekly_mileage.json"))?;
    let lifting_prs = read_json(&dir.join("lifting_prs.json"))?;
    let workout_summary = read_json(&dir.join("workout_summary.json"))?;

    let tmpl = s.templates.get_template("workouts.html")?;
    Ok(Html(tmpl.render(context! {
        running_stats,
        recent_runs,
        weekly_mileage,
        lifting_prs,
        workout_summary,
    })?))
}

fn read_json(path: &PathBuf) -> anyhow::Result<serde_json::Value> {
    let raw = fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("failed to read {}: {e}", path.display()))?;
    Ok(serde_json::from_str(&raw)?)
}
