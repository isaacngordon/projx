// src/commands/add_template.rs
use clap::ArgMatches;

pub fn add_template(matches: &ArgMatches) {
    let name = matches.get_one::<String>("name").unwrap();
    let file = matches.get_one::<String>("file").unwrap();
    println!("add template command executed with name: {} and file: {}", name, file);
}

