use std::io;
use crate::analyser::Analyser;
use crate::info::{Info, Loader};

const VERSION: &str = env!("CARGO_PKG_VERSION");

mod info;
mod analyser;

fn main() {
    println!("AnalSPbU v{}", VERSION);

    let stdin = io::stdin();

    let mut loader = Loader::new();
    loader.run();

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
                    print!("No abiturient found with the specified ID: \"{label}\"");
                    continue;
                }

                let Some(site_key) = analyser.admission().get(&id) else {
                    print!("Abiturient was not admitted to any site :(");
                    continue;
                };

                print!("And he/she/they goes to.. {}", site_key);
            }
            "refresh" => {
                let mut loader = Loader::new();
                loader.run();

                info = loader.release();

                analyser = Analyser::new();
                analyser.run(&info);

                print!("Successfully refreshed the data")
            }
            "points" => {
                for (site_key, _) in info.site_capacities() {
                    let (pass_score, last_admitted_place) = analyser.get_site_passing_info(&info, site_key);

                    println!("{site_key} -> {pass_score} | {last_admitted_place}")
                }
            }
            _ => println!("Unknown command")
        }
    }
}