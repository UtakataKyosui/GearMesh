use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OutputCache {
    entries: BTreeMap<String, String>,
}

impl OutputCache {
    pub fn load(path: &Path) -> Self {
        fs::read_to_string(path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    }

    pub fn has_changed(&self, output_path: &Path, content: &str) -> bool {
        let key = cache_key(output_path);
        let hash = content_hash(content);
        self.entries.get(&key) != Some(&hash)
    }

    pub fn update(&mut self, output_path: &Path, content: &str) {
        self.entries
            .insert(cache_key(output_path), content_hash(content));
    }

    pub fn persist(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let body = serde_json::to_string_pretty(self)
            .expect("OutputCache should always serialize to JSON");
        fs::write(path, body)
    }
}

pub fn cache_file(cache_dir: &Path) -> PathBuf {
    cache_dir.join("outputs.json")
}

fn cache_key(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn content_hash(content: &str) -> String {
    let digest = Sha256::digest(content.as_bytes());
    format!("{digest:x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_detects_content_changes() {
        let path = Path::new("generated/user.ts");
        let mut cache = OutputCache::default();

        assert!(cache.has_changed(path, "v1"));
        cache.update(path, "v1");
        assert!(!cache.has_changed(path, "v1"));
        assert!(cache.has_changed(path, "v2"));
    }
}
