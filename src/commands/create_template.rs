// src/commands/create_template.rs
use clap::ArgMatches;

pub fn create_template(matches: &ArgMatches) {
    let name = matches.get_one::<String>("name").unwrap();
    println!("create template command executed with name: {}", name);
}
