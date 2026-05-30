use jsondev::content;
use std::path::Path;

#[test]
fn all_markdown_files_parse() {
    let dirs = ["content/blog", "content/notes"];
    let mut errors = Vec::new();

    for dir in &dirs {
        let path = Path::new(dir);
        if !path.exists() {
            continue;
        }
        for entry in std::fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|e| e == "md") {
                let stem = path.file_stem().unwrap().to_string_lossy();
                if stem == "_index" {
                    continue;
                }
                if let Err(e) = content::parse_file(&path) {
                    errors.push(format!("{}: {e}", path.display()));
                }
            }
        }
    }

    if !errors.is_empty() {
        panic!("markdown parse errors:\n{}", errors.join("\n"));
    }
}

#[test]
fn single_pages_parse() {
    for path in ["content/bookshelf.md"] {
        content::parse_file(Path::new(path))
            .unwrap_or_else(|e| panic!("{path} failed to parse: {e}"));
    }
}
