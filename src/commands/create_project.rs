// src/commands/create_project.rs
use clap::ArgMatches;

pub fn create_project(matches: &ArgMatches) {
    let template_key = matches.get_one::<String>("template-key").unwrap();
    let project_name = matches.get_one::<String>("project-name").unwrap();
    println!("create project command executed with template-key: {} and project-name: {}", template_key, project_name);
}

