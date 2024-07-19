mod commands;

use clap::{Command, Arg};
use commands::create_project::create_project;
use commands::add_template::add_template;
use commands::create_template::create_template;

fn main() {
    let matches = Command::new("projx")
        .version("1.0")
        .about("Project Management CLI")
        .subcommand(
            Command::new("create")
                .about("Commands related to creation")
                .subcommand(
                    Command::new("project")
                        .about("Creates a new project")
                        .arg(Arg::new("template-key").required(true))
                        .arg(Arg::new("project-name").required(true))
                )
                .subcommand(
                    Command::new("template")
                        .about("Creates a new template")
                        .arg(Arg::new("name").required(true))
                )
        )
        .subcommand(
            Command::new("add")
                .about("Commands related to adding")
                .subcommand(
                    Command::new("template")
                        .about("Adds a new template")
                        .arg(Arg::new("name").required(true))
                        .arg(Arg::new("file").short('f').long("file").required(true))
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("create", create_matches)) => {
            match create_matches.subcommand() {
                Some(("project", project_matches)) => create_project(project_matches),
                Some(("template", template_matches)) => create_template(template_matches),
                _ => eprintln!("Unknown create command"),
            }
        }
        Some(("add", add_matches)) => {
            match add_matches.subcommand() {
                Some(("template", template_matches)) => add_template(template_matches),
                _ => eprintln!("Unknown add command"),
            }
        }
        _ => eprintln!("Unknown command"),
    }
}
