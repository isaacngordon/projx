use std::{fs, path::PathBuf};

// src/commands/mod.rs
pub mod template;
pub mod project;

/// Function to crawl a directory and add all files to a vector.
fn crawl_directory(dir: &PathBuf, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            crawl_directory(&path, files);
        } else {
            files.push(path);
        }
    }
}

/// Function to copy files to the destination directory while preserving the directory structure.
pub fn copy_files_to_destination(files: &Vec<PathBuf>, source_root: &PathBuf, dest_dir: &PathBuf) {
    for (index, file) in files.iter().enumerate() {
        let relative_path = if files.len() == 1 {
            PathBuf::from(file.file_name().unwrap())
        } else {
            PathBuf::from(
                file.strip_prefix(source_root).unwrap()
            )
        };

        let dest = dest_dir.join(&relative_path);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        match fs::copy(file, &dest) {
            Ok(_) => {println!("[{} of {}] Copied {} to {}", index + 1, files.len(), file.display(), dest.display());}
            Err(e) => {
                panic!("[{} of {}] Error copying {} to {}:\n{}", index + 1, files.len(), file.display(), dest.display(), e);
            }
        }
    }
}

#[cfg(test)]
mod tests;
