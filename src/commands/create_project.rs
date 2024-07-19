// src/commands/create_project.rs
use clap::ArgMatches;
use std::path::PathBuf;

pub fn create_project(matches: &ArgMatches) {
    let template_key = matches.get_one::<String>("template-key").unwrap();
    let project_name = matches.get_one::<String>("project-name").unwrap();
    let destination = matches.get_one::<String>("destination");

    // Ensure the template directory exists
    let path_to_templates = if cfg!(debug_assertions) {
        PathBuf::from("src/templates")
    } else {
        let mut home_dir = dirs::home_dir().unwrap();
        home_dir.push(".projx/templates");
        home_dir
    };

    let template_dir = path_to_templates.join(template_key);

    println!("Looking for template directory at: {}", template_dir.display());
    if !template_dir.exists() {
        eprintln!("Error: Template directory '{}' does not exist.", template_key);
        std::process::exit(1);
    }

    // Determine the destination directory based on the provided argument or default paths.
    let destination_dir = if let Some(dest) = destination {
        PathBuf::from(dest)
    } else if cfg!(debug_assertions) {
        eprintln!("Error: Destination must be provided in development mode.");
        std::process::exit(1);
    } else {
        let mut home_dir = dirs::home_dir().unwrap();
        home_dir.push(".projx/projects");
        home_dir.join(project_name)
    };

    println!("create project command executed with template-key: {}, project-name: {}, and destination: {}", template_key, project_name, destination_dir.display());
    println!("Template directory path: {}", template_dir.display());
    println!("Destination directory path: {}", destination_dir.display());
}

