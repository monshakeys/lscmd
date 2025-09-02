use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn scan_directory<P: AsRef<Path>>(root_path: P) -> Result<Vec<PathBuf>> {
    let root = root_path.as_ref();
    
    if !root.exists() {
        return Err(anyhow::anyhow!("目錄 '{}' 不存在。", root.display()));
    }

    let mut shell_files = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s == "sh")
                .unwrap_or(false)
        })
    {
        shell_files.push(entry.into_path());
    }

    shell_files.sort();

    if shell_files.is_empty() {
        return Err(anyhow::anyhow!("在 '{}' 中找不到任何 .sh 檔案。", root.display()));
    }

    Ok(shell_files)
}