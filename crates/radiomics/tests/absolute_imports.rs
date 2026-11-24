//! Project rule: absolute paths only.
//!
//! Every `use` and inline path in this crate and in the PyO3 binding crate
//! (`crates/radiomics-py/src`) starts with `crate::`, `$crate::` or an
//! external crate. `super::`, `self::` and bare paths to local modules (`use config::...`)
//! are rejected; `pub(super)` visibility and `mod name;` declarations are fine.

use std::fs;
use std::path::{Path, PathBuf};

fn rust_files(dir: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_files(&path, files);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
}

/// Modules declared in this file with `mod name;` or `mod name {`.
fn local_modules(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|line| {
            let line = line.trim_start();
            let line = line
                .strip_prefix("pub(crate) ")
                .or_else(|| line.strip_prefix("pub(super) "))
                .or_else(|| line.strip_prefix("pub "))
                .unwrap_or(line);
            let name = line.strip_prefix("mod ")?;
            let end = name.find([';', ' ', '{'])?;
            Some(name[..end].to_string())
        })
        .collect()
}

fn relative_paths(text: &str) -> Vec<(usize, String)> {
    let modules = local_modules(text);
    let mut found = Vec::new();
    let mut absolute_use = false;
    for (number, line) in text.lines().enumerate() {
        for keyword in ["super::", "self::"] {
            let mut start = 0;
            while let Some(offset) = line[start..].find(keyword) {
                let at = start + offset;
                let before = line[..at].chars().next_back();
                let identifier_char =
                    before.is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '$');
                if !identifier_char
                    && !line[..at].ends_with("pub(")
                    && !line[..at].ends_with("pub(in ")
                {
                    found.push((number + 1, line.trim().to_string()));
                }
                start = at + keyword.len();
            }
        }
        // A path that starts with a module declared in this file (`use bounds::x;` or an
        // inline `bounds::include(...)`) is relative. Lines inside a `use crate::...`
        // statement are absolute even when a group names a submodule (`{bounds::x}`).
        let trimmed = line.trim_start();
        let after_use = trimmed
            .strip_prefix("pub(crate) use ")
            .or_else(|| trimmed.strip_prefix("pub use "))
            .or_else(|| trimmed.strip_prefix("use "));
        if let Some(path) = after_use {
            absolute_use = !path.contains(';') && path.starts_with("crate::");
            if path.starts_with("crate::") {
                continue;
            }
        } else if absolute_use {
            absolute_use = !line.contains(';');
            continue;
        }
        let relative_module = modules.iter().any(|module| {
            let pattern = format!("{module}::");
            line.match_indices(&pattern).any(|(at, _)| {
                let before = line[..at].chars().next_back();
                !before.is_some_and(|c| c.is_alphanumeric() || c == '_' || c == ':' || c == '$')
            })
        });
        if relative_module {
            found.push((number + 1, line.trim().to_string()));
        }
    }
    found
}

#[test]
fn sources_and_tests_use_absolute_paths_only() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    rust_files(&root.join("src"), &mut files);
    rust_files(&root.join("tests"), &mut files);
    rust_files(&root.join("../radiomics-py/src"), &mut files);
    files.sort();
    let mut violations = Vec::new();
    for path in &files {
        if path.ends_with("tests/absolute_imports.rs") {
            continue; // this checker spells the forbidden forms out
        }
        let text = fs::read_to_string(path).unwrap();
        for (line, code) in relative_paths(&text) {
            let shown = path.display();
            violations.push(format!("{shown}:{line}: {code}"));
        }
    }
    assert!(files.len() > 50, "scanned only {} files", files.len());
    assert!(
        violations.is_empty(),
        "relative paths; use crate::... instead:\n{}",
        violations.join("\n")
    );
}

#[test]
fn the_checker_flags_relative_forms() {
    let sample = "mod inner;\nuse super::x;\npub use self::y;\nuse inner::z;\npub(super) fn f() {}\nlet v = super::g();\nuse crate::ok;\nlet w = inner::h(1);\nlet a = crate::m::inner::h(1);\nuse crate::m::{\n    inner::k,\n};\nlet b = (inner::j)();\n";
    let lines: Vec<usize> = relative_paths(sample).into_iter().map(|(n, _)| n).collect();
    assert_eq!(lines, vec![2, 3, 4, 6, 8, 13]);
}
