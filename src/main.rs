/**
 * File              : main.rs
 * Author            : Amar Lakshya <amar.lakshya@xaviers.edu.in>
 * Date              : 09.07.2019
 * Last Modified Date: 09.07.2019
 * Last Modified By  : Amar Lakshya <amar.lakshya@xaviers.edu.in>
 */
#[macro_use]

extern crate clap;
use clap::App;
use std::process::Command;

fn run_command(command: &str, path: &str) -> bool {
    Command::new(command)
        .arg(path)
        .output()
        .expect("")
        .status
        .success()
}

fn list_flags(matches: &clap::ArgMatches, path: &std::string::String) {
    run_command("/usr/bin/touch", &format!("{}/{}", path, "flag.md"));
}

fn set_writeups(matches: &clap::ArgMatches) {}

fn create_standard(path: &std::string::String) {
    let standard_challenges = [
        "crypto",
        "hardware",
        "misc",
        "pwn",
        "reversing",
        "sandbox",
        "web",
    ];
    println!("Creating standard jeopardy style directories...");
    for v in standard_challenges.iter() {
        run_command("/usr/bin/mkdir", &format!("{}/{}", path, v));
    }
}

fn set_categories(matches: &clap::ArgMatches, path: &std::string::String) {
    match matches.value_of("categories").unwrap_or("standard") {
        "custom" => println!("custom"),
        "standard" => {
            println!("Setting categories as standard");
            create_standard(&path);
        }
        _ => println!("Couldn't parse the option, Setting categories as standard"),
    }
}
fn set_format(matches: &clap::ArgMatches, path: &std::string::String) -> () {
    match matches.value_of("format").unwrap_or("jeopardy") {
        "attack-defence" => {
            println!("Setting format as attack-defence style");
        }
        _ => {
            println!("Setting CTF as jeopardy style");
            set_categories(matches, &path);
        }
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        let name = matches.value_of("name").unwrap();
        let directory = matches.value_of("directory").unwrap();

        let path = [directory, name].join("/");
        if matches.is_present("directory") {
            print!("Creating {:?} Directory in {:?}...", name, directory);
            println!(
                "{:?}",
                Command::new("/usr/bin/git")
                    .arg("init")
                    .arg(&path)
                    .output()
                    .expect("")
                    .status
                    .success()
            );
        }
        set_format(matches, &path);
        set_writeups(matches);
        list_flags(matches, &path);
    }
}
