use std::{fs, iter::zip, path::PathBuf, str::FromStr};

use clap::Parser;
use dialoguer::Confirm;
use log::{error, info, trace};

pub mod data;
use data::{Entry, BOP};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, help = "ballast csv file")]
    ballast: String,

    #[arg(short, long, help = "output file, defaults to bop.json")]
    output: Option<String>,

    #[arg(short, long, help = "verbose logging, use to make sure it parsed correctly")]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    let log_level = if args.verbose {
        log::LevelFilter::Trace
    } else {
        log::LevelFilter::Info
    };
    env_logger::builder().filter_level(log_level).init();

    let path = PathBuf::from(if let Some(target) = args.output {
        target
    } else {
        "bop.json".to_string()
    });

    if path.is_dir() {
        error!("Output path is a folder, please point at a File!");
        return;
    }

    if path.exists() {
        if !Confirm::new()
            .with_prompt(format!(
                "File {} already exists. Override?",
                path.clone().to_str().expect("it is a string")
            ))
            .default(false)
            .interact()
            .unwrap_or(false)
        {
            info!("Unable to Save, Exiting...");
            return;
        }
    }

    if let Some(res) = parse_csv(&args.ballast) {
        // Removing entries with no bop adjustment
        let mut entries = Vec::<Entry>::with_capacity(res.len());
        for item in res {
            if item.ballast_kg.is_some() || item.restrictor.is_some() {
                entries.push(item);
            }
        }

        if let Ok(json) = serde_json::to_string_pretty(&BOP { entries }) {
            if fs::write(&path, json).is_ok() {
                info!(
                    "Finished writing to {}",
                    path.to_str().expect("it is a string")
                );
                return;
            }
        }

        error!("Failed to write {}", path.to_str().expect("it is a string"));
    } else {
        error!("Unable to parse csv, exiting...");
    }
}

pub fn parse_csv(csv_file_path: &String) -> Option<Vec<Entry>> {
    let path = PathBuf::from_str(csv_file_path).ok()?;

    if !path.is_file() {
        error!("File does not exist!");
        return None;
    }

    info!("Loading balast file {}", csv_file_path);
    let file = fs::read_to_string(path).ok()?;

    let mut file = file.split("\n");
    let mut toprow = file.next()?.split(",");
    toprow.next()?;

    let mut tracks: Vec<String> = vec![];
    for element in toprow {
        trace!("Found track: {}", element);
        tracks.push(element.to_string()); //TODO validate tracks
    }
    info!("Found {} tracks", tracks.len());

    let mut entries: Vec<Entry> = vec![];
    let mut count = 0;
    for car in file {
        if !car.is_empty() {
            let mut row = car.split(",");
            if let Some(model) = validate_car_model(row.next()) {
                // Reading the track entries
                let iter = zip(row, tracks.iter());
                for (element, track) in iter {
                    entries.push(create_ballast_entry(element, model, track));
                }
                count += 1;
            }
        }
    }
    info!("Parsed {} cars", count);

    Some(entries)
}

fn create_ballast_entry(element: &str, model: u32, track: &String) -> Entry {
    let weight = if let Ok(weight) = i32::from_str(element) {
        if weight != 0 {
            Some(weight)
        } else {
            None
        }
    } else {
        None
    };
    trace!(
        "car {} at {}: {}kg",
        model,
        track,
        if let Some(t) = weight { t } else { 0 }
    );

    let entry = Entry {
        track: track.clone(),
        car_model: model,
        ballast_kg: weight,
        restrictor: None,
    };
    entry
}

pub fn validate_car_model(model_str: Option<&str>) -> Option<u32> {
    if let Some(text) = model_str {
        if let Some(id) = u32::from_str(text).ok() {
            trace!("Found car {}:", id); // TODO, print also which car that would be
            return Some(id);
        } else {
            error!("Unable to parse car model '{}', skipping", text);
            return None;
        }
    }

    None
}