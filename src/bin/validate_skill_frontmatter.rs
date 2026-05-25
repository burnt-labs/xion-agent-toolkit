//! Dev/CI tool: validate SKILL.md frontmatter for Codex and Agent Skills loaders.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use regex::Regex;
use serde_yaml::Value;

const MAX_DESC_LEN: usize = 1024;

fn usage() -> &'static str {
    "Usage: validate-skill-frontmatter [SKILLS_DIR]\n\n\
     Default SKILLS_DIR is <repo>/skills next to Cargo.toml."
}

fn extract_frontmatter(text: &str) -> Result<String, String> {
    if !text.starts_with("---\n") && !text.starts_with("---\r\n") {
        return Err("missing opening ---".into());
    }

    let re = Regex::new(r"\A---[ \t]*\r?\n([\s\S]*?)\r?\n---[ \t]*\r?(?:\n|\z)")
        .expect("frontmatter regex compiles");
    let caps = re
        .captures(text)
        .ok_or_else(|| {
            "missing or invalid closing --- delimiter (must be exactly --- on its own line)"
                .to_string()
        })?;
    Ok(caps.get(1).expect("capture group 1").as_str().to_string())
}

fn validate_file(path: &Path) -> Result<usize, String> {
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let block = extract_frontmatter(&text)?;

    let value: Value =
        serde_yaml::from_str(&block).map_err(|e| format!("invalid YAML: {e}"))?;

    let map = value
        .as_mapping()
        .ok_or_else(|| "frontmatter must be a YAML mapping".to_string())?;

    let name = map
        .get(Value::from("name"))
        .ok_or_else(|| "missing name field".to_string())?;
    if !name.is_string() {
        return Err("name must be a string".into());
    }

    let desc = map
        .get(Value::from("description"))
        .ok_or_else(|| "missing description field".to_string())?;
    let desc_str = desc
        .as_str()
        .ok_or_else(|| {
            "description must be a string (use quoted or block scalar, not a mapping/list)"
                .to_string()
        })?;

    Ok(desc_str.len())
}

fn check_skill(path: &Path) -> Result<(), String> {
    let name = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    let len = validate_file(path)?;
    if len > MAX_DESC_LEN {
        return Err(format!(
            "description length {len} exceeds {MAX_DESC_LEN}"
        ));
    }
    println!("OK   {name} ({len} chars)");
    Ok(())
}

fn main() -> ExitCode {
    let skills_dir = match env::args().nth(1) {
        Some(dir) => PathBuf::from(dir),
        None => PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("skills"),
    };

    if env::args().any(|a| a == "-h" || a == "--help") {
        eprintln!("{}", usage());
        return ExitCode::SUCCESS;
    }

    if !skills_dir.is_dir() {
        eprintln!("ERROR: skills directory not found: {}", skills_dir.display());
        return ExitCode::from(2);
    }

    let mut fail = false;
    println!("Validating skills under {}", skills_dir.display());

    let mut entries: Vec<_> = match fs::read_dir(&skills_dir) {
        Ok(rd) => rd.filter_map(Result::ok).collect(),
        Err(e) => {
            eprintln!("ERROR: cannot read {}: {e}", skills_dir.display());
            return ExitCode::from(2);
        }
    };
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let skill_md = entry.path().join("SKILL.md");
        if !skill_md.is_file() {
            continue;
        }
        if let Err(err) = check_skill(&skill_md) {
            let name = entry.file_name().to_string_lossy().into_owned();
            eprintln!("FAIL {name}: {err}");
            fail = true;
        }
    }

    if fail {
        ExitCode::FAILURE
    } else {
        println!("All skill frontmatter checks passed.");
        ExitCode::SUCCESS
    }
}
