//! Read-only filesystem browsing for the working-directory file tree.
//!
//! This is the harness's own minimal fs view (one directory level at a time),
//! distinct from any agent file access. It stays read-only by design: the slice
//! never mutates the disk through this path.

use serde::Serialize;
use std::path::Path;

/// A single directory entry surfaced to the file tree.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DirEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
}

/// List the immediate children of `dir`, directories first then files, each
/// group sorted by name. Hidden entries (dotfiles) are included.
pub fn list_dir(dir: &Path) -> std::io::Result<Vec<DirEntry>> {
    let mut entries: Vec<DirEntry> = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
        entries.push(DirEntry {
            name: entry.file_name().to_string_lossy().into_owned(),
            path: path.to_string_lossy().into_owned(),
            is_dir,
        });
    }
    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_dirs_before_files_sorted() {
        let tmp = std::env::temp_dir().join(format!("lf-files-test-{}", std::process::id()));
        std::fs::create_dir_all(tmp.join("zsub")).unwrap();
        std::fs::create_dir_all(tmp.join("asub")).unwrap();
        std::fs::write(tmp.join("b.txt"), "x").unwrap();
        std::fs::write(tmp.join("a.txt"), "x").unwrap();

        let entries = list_dir(&tmp).unwrap();
        let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, vec!["asub", "zsub", "a.txt", "b.txt"]);
        assert!(entries[0].is_dir);
        assert!(!entries[2].is_dir);

        std::fs::remove_dir_all(&tmp).unwrap();
    }
}
