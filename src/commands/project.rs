// src/commands/create_project.rs
use super::{
    copy_files_to_destination, crawl_directory,
    template::{DEBUG_TEMPLATES_PATH, RELEASE_TEMPLATES_PATH},
};
use std::path::PathBuf;

pub const DEBUG_PROJECTS_PATH: &str = "src/projects";
pub const RELEASE_PROJECTS_PATH: &str = ".projx/projects";

pub fn create_project(
    template_key: &str,
    project_name: &str,
    destination: Option<&str>,
) -> Result<(), String> {
    // Ensure the template directory exists (based on the provided template key, and the current environment).
    let path_to_templates = if cfg!(debug_assertions) {
        PathBuf::from(DEBUG_TEMPLATES_PATH)
    } else {
        let mut home_dir = dirs::home_dir().unwrap();
        home_dir.push(RELEASE_TEMPLATES_PATH);
        home_dir
    };

    let template_dir = path_to_templates.join(template_key);

    println!(
        "Looking for template directory at: {}",
        template_dir.display()
    );
    if !template_dir.exists() {
        return Err(format!(
            "Template directory '{}' does not exist.",
            template_key
        ));
    }

    // Determine the destination directory based on the provided argument or default paths.
    // If the destination argument is provided, use it. Otherwise, use the default path based on the environment.
    let destination_dir = if let Some(dest) = destination {
        PathBuf::from(dest)
    } else if cfg!(debug_assertions) {
        PathBuf::from(DEBUG_PROJECTS_PATH).join(project_name)
    } else {
        let mut home_dir = dirs::home_dir().unwrap();
        home_dir.push(RELEASE_PROJECTS_PATH);
        home_dir.join(project_name)
    };

    println!("create project command executed with template-key: {}, project-name: {}, and destination: {}", template_key, project_name, destination_dir.display());
    println!("Template directory path: {}", template_dir.display());
    println!("Destination directory path: {}", destination_dir.display());

    // Ensure the destination directory does not already exist.
    if destination_dir.exists() {
        return Err(format!(
            "Destination directory '{}' already exists.",
            destination_dir.display()
        ));
    }

    // Copy the template directory to the destination directory.
    copy_files_to_project(&template_dir, &destination_dir);

    Ok(())
}

fn copy_files_to_project(template_root: &PathBuf, project_root: &PathBuf) {
    let mut files = Vec::new();
    crawl_directory(template_root, &mut files);
    println!("Files: {:?}", files);
    copy_files_to_destination(&files, template_root, project_root);
}
