use std::io;
use crate::analyser::Analyser;
use crate::info::{Loader};

const VERSION: &str = env!("CARGO_PKG_VERSION");

mod info;
mod analyser;

fn main() {
    println!("AnalSPbU v{}", VERSION);

    let stdin = io::stdin();

    let mut loader = Loader::new();
    loader.run(true);

    let mut info = loader.release();

    let mut analyser = Analyser::new();
    analyser.run(&info);

    println!("Successfully refreshed the data");

    loop {
        println!("Send a command...");

        let mut input = "".to_string();
        stdin.read_line(&mut input).unwrap();

        let mut input = input.split_whitespace();

        let cmd = input.next();

        if cmd.is_none() {
            continue;
        }

        let cmd = cmd.unwrap().to_lowercase().to_string();

        let mut label = "".to_string();

        for entry in input {
            label += " ";
            label += entry.to_lowercase().as_ref();
        }

        match cmd.as_str() {
            "info" => {
                if label.is_empty() {
                    println!("Please specify the ID");
                }

                let id = label.trim().to_string();

                if !info.is_abit_registered(&id) {
                    println!("No abiturient found with the specified ID: \"{label}\"");
                    continue;
                }

                let Some(site_key) = analyser.admission().get(&id) else {
                    println!("Abiturient was not admitted to any site :(");
                    continue;
                };

                println!("And he/she/they go(es) to.. {}", site_key);
            }
            "refresh" => {
                let mut with_quotas = true;

                if !label.is_empty() {
                    if label.trim() == "--ignore-quotas" {
                        with_quotas = false;
                        println!("Quotas will be ignored.");
                    } else {
                        println!("Unknown option `{}`. Write `help` to view available options.", label.trim());
                    }
                }

                let mut loader = Loader::new();
                loader.run(with_quotas);

                info = loader.release();

                analyser = Analyser::new();
                analyser.run(&info);

                println!("Successfully refreshed the data")
            }
            "points" => {
                for (site_key, places) in info.site_capacities() {
                    let (pass_score, last_admitted_place) = analyser.get_site_passing_info(&info, site_key);

                    println!("{site_key} [{places} places] -> {pass_score}pts | #{last_admitted_place}")
                }
            }
            "help" => {
                println!("Available commands:");
                println!("help                            See available commands");
                println!("info <ID>                       Find out where the applicant with the given ID gets to");
                println!("points                          See the information about passing points");
                println!("refresh [--ignore-quotas]       Refresh the data (optionally without considering quotas)");
            }
            _ => println!("Unknown command. Write `help` to see available commands.")
        }
    }
}