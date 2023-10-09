use std::{fs, iter::zip, path::PathBuf, str::FromStr};

use clap::Parser;
use dialoguer::Confirm;
use log::{error, info, trace};

pub mod data;
use data::{Entry, BOP, TRACKS, CARS};


#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, help = "ballast csv file (required)")]
    ballast: Option<String>, // this is an option so we can have the helper outputs for carmodel- and tracklists

    #[arg(short, long, help = "output file, defaults to bop.json")]
    output: Option<String>,

    #[arg(
        short,
        long,
        help = "verbose logging, use to make sure it parsed correctly"
    )]
    verbose: bool,

    #[arg(long, help = "list all tracks and exit")]
    list_tracks: bool,

    #[arg(long, help = "list all carmodel ids and exit")]
    list_carmodels: bool,
}

fn main() {
    let args = Args::parse();
    let log_level = if args.verbose {
        log::LevelFilter::Trace
    } else {
        log::LevelFilter::Info
    };
    env_logger::builder().filter_level(log_level).init();

    // Handling the lists
    if args.list_carmodels {
        println!("All Cars (printed pseudo alphabetical order, this is the order the lookup operation uses):");
        for (id, item) in CARS {
            println!("{}: {}", item, id);
        }
        return;
    }
    if args.list_tracks {
        println!("All Tracks:");
        for item in TRACKS {
            println!("{}", item);
        }
        return;
    }

    // Verifying that ballast path is present
    if args.ballast.is_none() {
        error!("Ballast is required! See --help for futher info");
        return;
    }

    // Setting output folder
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

    if let Some(res) = parse_csv(
        args.ballast
            .expect("We verified that ballast flag is present"),
    ) {
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

pub fn parse_csv(csv_file_path: String) -> Option<Vec<Entry>> {
    let path = PathBuf::from_str(&csv_file_path).ok()?;

    if !path.is_file() {
        error!("File does not exist!");
        return None;
    }

    info!("Loading balast file {}", &csv_file_path);
    let file = fs::read_to_string(path).ok()?;

    let mut file = file.split("\n");
    let mut toprow = file.next()?.split(",");
    toprow.next()?;

    let mut tracks: Vec<Option<String>> = vec![];
    for element in toprow {
        if let Some(track) = validate_track(element) {
            tracks.push(Some(track));
        } else {
            error!("Unable to parse track '{}', skipping", element);
            tracks.push(None); // This has to be an option, as we need to later be able to keep the columns intact for the weights
        }
    }
    info!("Found {} tracks", tracks.len());

    let mut entries: Vec<Entry> = vec![];
    let mut count = 0;
    for car in file {
        let test = car.replace(",", "");
        if !test.trim().is_empty() {
            let mut row = car.split(",");
            if let Some(model) = validate_car_model(row.next()) {
                // Reading the track entries
                let iter = zip(row, tracks.iter());
                for (element, track) in iter {
                    if let Some(track) = track { // columns with bad headers still contain weights, we skip those but keep iterating to keep the order
                        entries.push(create_ballast_entry(element, model, track));
                    }
                }
                count += 1;
            }
        }
    }
    info!("Parsed {} cars", count);

    Some(entries)
}

fn create_ballast_entry(weight_string: &str, model: u32, track: &String) -> Entry {
    let weight_string = weight_string.trim();
    let weight_string = if weight_string.is_empty() {
        // this is done so we can error when the parse failed without erroring on empty
        "0"
    } else {
        weight_string
    };

    let car_name = get_car_name_from_id(model).unwrap_or(model.to_string());

    let weight = if let Ok(weight) = i32::from_str(weight_string) {
        if weight == 0 {
            None // Allows us to drop the entry later when excluding those without any adjustments
        } else if weight > 40 {
            error!(
                "Weight for car {} at track {} exceeded 40kg ({}), using 40kg",
                car_name, track, weight
            );
            Some(40)
        } else if weight < -40 {
            error!(
                "Weight for car {} at track {} exceeded -40kg ({}), using -40kg",
                car_name, track, weight
            );
            Some(-40)
        } else {
            Some(weight)
        }
    } else {
        error!(
            "Failed to parse weight for car {} at track {}. String was {}. Skipping",
            car_name, track, weight_string
        );
        None
    };
    trace!(
        "car {} ({}) at {}: {}kg",
        car_name,
        model,
        track,
        weight.unwrap_or(0)
    );

    let entry = Entry {
        track: track.clone(),
        car_model: model,
        ballast_kg: weight,
        restrictor: None,
    };
    entry
}

pub fn validate_track(track_str: &str) -> Option<String> {
    let track_str = track_str.replace(" ", "_").to_lowercase().replace("bathurst", "mount_panorama");

    for item in TRACKS {
        if item.eq_ignore_ascii_case(track_str.as_str()) {
            trace!("Found Track {}", item);
            return Some(item.to_string());
        }
    }
    
    None
}

pub fn validate_car_model(model_str: Option<&str>) -> Option<u32> {
    if let Some(text) = model_str {
        // Finding based on ID
        if let Some(id) = u32::from_str(text).ok() {
            if let Some(car_name) =  get_car_name_from_id(id) {
                info!("Found car {} ({})", car_name, id); 
                return Some(id);
            } else {
                error!("No car is known to have id {}", id)
            }
        }

        // We try to find the car based on the name, specifically we turn the text into tokens and then see if one carname contains all tokens
        let keywords:Vec<&str> = text.split(" ").filter(|sample| !sample.trim().is_empty()).collect();
        if !keywords.is_empty() {
            for (id, car_name) in CARS {
                let car_name_compare = car_name.to_lowercase();
                
                let mut is_it = true;
                for key in &keywords {
                    let key = key.trim().to_lowercase();
    
                    if !car_name_compare.contains(key.as_str()) {
                        is_it = false;
                        break;
                    }
                }

                if is_it {
                    info!("Found car {} ({})", car_name, id);
                    return Some(id);
                }
            }
        }


        error!("Unable to parse car model '{}', skipping", text);
        return None;
    }

    None
}

pub fn get_car_name_from_id(car_id: u32) -> Option<String> {
    // Couldn't we put all ids and names into a map? Yes, but considering that we have only about 50, this is not a performance issue
    for (id, name) in CARS { 
        if id == car_id {
            return Some(name.to_string());
        }
    }

    None
}
