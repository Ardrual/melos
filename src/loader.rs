use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a loaded source with its content and base path
#[derive(Debug)]
pub struct LoadedSource {
    /// The concatenated source code
    pub source: String,
    /// The base path (file path for single file, directory path for multi-file)
    pub base_path: PathBuf,
}

/// Load Melos source from a file or directory.
///
/// If `path` is a file, reads it directly.
/// If `path` is a directory, finds all .mel files, sorts them (score.mel first,
/// then alphabetically), and concatenates them.
pub fn load_source(path: &Path) -> Result<LoadedSource> {
    if path.is_file() {
        load_single_file(path)
    } else if path.is_dir() {
        load_directory(path)
    } else {
        bail!("Path does not exist: {:?}", path)
    }
}

fn load_single_file(path: &Path) -> Result<LoadedSource> {
    let source = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {:?}", path))?;

    Ok(LoadedSource {
        source,
        base_path: path.to_path_buf(),
    })
}

fn load_directory(dir: &Path) -> Result<LoadedSource> {
    // Find all .mel files in the directory
    let mut mel_files: Vec<PathBuf> = fs::read_dir(dir)
        .with_context(|| format!("Failed to read directory: {:?}", dir))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file() && path.extension().map_or(false, |ext| ext == "mel")
        })
        .collect();

    if mel_files.is_empty() {
        bail!("No .mel files found in directory: {:?}", dir);
    }

    // Sort: score.mel first, then alphabetically by filename
    mel_files.sort_by(|a, b| {
        let a_name = a.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let b_name = b.file_name().and_then(|n| n.to_str()).unwrap_or("");

        match (a_name, b_name) {
            ("score.mel", _) => std::cmp::Ordering::Less,
            (_, "score.mel") => std::cmp::Ordering::Greater,
            _ => a_name.cmp(b_name),
        }
    });

    // Read and concatenate all files
    let mut combined = String::new();
    for (i, file_path) in mel_files.iter().enumerate() {
        let content = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {:?}", file_path))?;

        combined.push_str(&content);

        // Add newline between files if the content doesn't end with one
        if i < mel_files.len() - 1 && !content.ends_with('\n') {
            combined.push('\n');
        }
    }

    Ok(LoadedSource {
        source: combined,
        base_path: dir.to_path_buf(),
    })
}
