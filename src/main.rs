use std::collections::HashSet;
use std::io;
use reqwest::get;
use crate::analyzer::Analyzer;
use crate::drop_analyzer::DropAnalyzer;
use crate::info::{Loader, SiteKey};

const VERSION: &str = env!("CARGO_PKG_VERSION");

mod info;
mod analyzer;
mod drop_analyzer;

fn main() {
    println!("AnalSPbU v{}", VERSION);

    let needle = math_filter();

    let stdin = io::stdin();

    let mut loader = Loader::new();
    loader.run();
    loader.fetch_quotas();

    let mut info = loader.release();

    let mut analyser = Analyzer::new();
    analyser.run(&info);

    println!("Successfully refreshed the data");

    loop {
        let mut input = "".to_string();
        let Ok(_) = stdin.read_line(&mut input) else {
            continue;
        };

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
                    continue;
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
            "rescale" => {
                if label.is_empty() {
                    println!("Please specify the value");
                    continue;
                }

                let Ok(val) = label.trim().parse::<u32>() else {
                    println!("Please specify a valid value");
                    continue;
                };

                info.rescale(val);

                analyser = Analyzer::new();
                analyser.run(&info);

                println!("Rescaled the capacities to {val}%")
            }
            "refresh" => {
                let mut with_quotas = true;

                // if !label.is_empty() {
                //     if label.trim() == "--ignore-quotas" {
                //         with_quotas = false;
                //         println!("Quotas will be ignored.");
                //     } else {
                //         println!("Unknown option `{}`. Write `help` to view available options.", label.trim());
                //     }
                // }

                let mut loader = Loader::new();
                loader.run();
                loader.fetch_quotas();

                info = loader.release();

                analyser = Analyzer::new();
                analyser.run(&info);

                println!("Successfully refreshed the data")
            }
            "points" => {
                for (site_key, places) in info.site_capacities() {
                    let (pass_score, last_admitted_place) = analyser.get_site_passing_info(&info, site_key);

                    println!("{site_key} [{places} places] -> {pass_score} | #{last_admitted_place}")
                }
            }
            "drop_analyze" => {
                macro_rules! wa {
                    () => {
                        println!("Wrong arguments. Write `help` to see details");
                        continue;
                    }
                }

                macro_rules! get_int {
                    ($spl:ident) => {
                        {
                            let Some(x) = $spl.next() else {
                                wa!();
                            };

                            let Ok(x) = x.parse::<u32>() else {
                                wa!();
                            };

                            x
                        }
                    }
                }

                let mut split = label.trim().split_whitespace();

                let k = get_int!(split);

                let percent = get_int!(split);

                let Some(range) = split.next() else {
                    wa!();
                };

                let mut split2 = range.split(":");

                let start = get_int!(split2);
                let end = get_int!(split2);

                let mut id = "".to_string();

                while let Some(part) = split.next() {
                    id += " ";
                    id += part;
                }

                let id = id.trim().to_string();

                if !id.is_empty() {
                    if !info.is_abit_registered(&id) {
                        println!("No abiturient found with the specified ID: \"{id}\"");
                        continue;
                    }

                    let mut analyzer = DropAnalyzer::new(percent, (start, end));
                    let (site_times, none_times) = analyzer.run_personal(&info, &id, k);

                    let mut out = vec![];

                    if none_times > 0 {
                        out.push(("Was not admitted", none_times));
                    }

                    for (site, times) in &site_times {
                        out.push((site.as_str(), *times));
                    }

                    out.sort_by_key(|(_, t)| *t as i32 * -1);

                    println!("Simulated {k} times:");
                    for (to, times) in out {
                        println!("{to} -> {:.2}%", times as f64 / k as f64 * 100.);
                    }

                    continue;
                }

                let mut analyzer = DropAnalyzer::new(percent, (start, end));
                let results = analyzer.run(&info, k);

                let mut out = vec![];

                for (site_key, result) in &results {
                    if !needle.contains(site_key) {
                        continue;
                    }

                    let result = ((*result * 100.).round()) / 100.;

                    out.push((site_key, result));
                }

                out.sort_by_key(|(_, k)| ((*k * 100.).floor() as i64) * -1);

                for (site, result) in out {
                    println!("{site} -> {result}");
                }
            }
            "help" => {
                println!("Available commands:");
                println!("help                                 See available commands");
                println!("info <ID>                            Find out where the applicant with the given ID gets to");
                println!("points                               See the information about passing points");
                println!("refresh                              Refresh the data (optionally without considering quotas)");
                println!("rescale <value>                      Rescale the capacities of the sites to <value>%");
                println!("drop_analyze <n> <from>:<to> [ID]    Drop <n>% of random abiturients in given score range");

            }
            _ => println!("Unknown command. Write `help` to see available commands.")
        }
    }
}

fn math_filter() -> HashSet<SiteKey> {
    let mut set = HashSet::new();

    macro_rules! add {
        ($s:expr) => {
            set.insert($s.to_string());
        }
    }

    add!("СВ.5000.2023 Математика");
    add!("СВ.5003.2023 Программирование и информационные технологии");
    add!("СВ.5212.2023 Искусственный интеллект и наука о данных");
    add!("СМ.5089.2023 Фундаментальная механика");
    add!("СВ.5112.2023 Инженерно-ориентированная физика");
    add!("СВ.5190.2023 Большие данные и распределенная цифровая платформа");
    add!("СВ.5009.2023 Прикладные физика и математика");
    add!("СВ.5001.2023 Математика и компьютерные науки");
    add!("СВ.5189.2023 Науки о данных");
    add!("СВ.5011.2023 Физика");
    add!("СВ.5008.2023 Механика и математическое моделирование");
    add!("СВ.5078.2023 Прикладная информатика в области искусств и гуманитарных наук");
    add!("СВ.5156.2023 Современное программирование");
    add!("СВ.5213.2023 Прикладная математика, программирование и искусственный интеллект");
    add!("СВ.5071.2023 Бизнес-информатика");
    add!("СВ.5005.2023 Прикладная математика, фундаментальная информатика и программирование");
    add!("СВ.5080.2023 Программная инженерия");
    add!("СВ.5162.2023 Технологии программирования");
    add!("СМ.5088.2023 Фундаментальная математика");
    add!("СВ.5164.2023 Прикладные компьютерные технологии");

    set
}