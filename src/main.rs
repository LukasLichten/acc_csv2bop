use std::{fs, iter::zip, path::PathBuf, str::FromStr, collections::HashMap};

use clap::Parser;
use dialoguer::Confirm;
use log::{error, info, trace};

pub mod data;
use data::{Entry, BOP, CARS, TRACKS};

#[cfg(test)]
mod test;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, help = "ballast csv file (required)")]
    ballast: Option<String>, // this is an option so we can have the helper outputs for carmodel and tracklists

    #[arg(short, long, help = "restrictor csv file (optional)")]
    restrictor: Option<String>,

    #[arg(short, long, help = "output file, defaults to bop.json / ballast.csv")]
    output: Option<String>,

    #[arg(short, long, help = "A bop.json to parse to CSV file(s)")]
    json: Option<String>,

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

    // bop 2 csv
    if let Some(bop) = args.json {
        if bop2csv(bop, args.output).is_none() {
            error!("Failed to parse bop to csv, exiting...");
        }
        info!("Finished Writing");
        return;
    }

    // Verifying that ballast path is present
    let ballast_file = if let Some(bal) = args.ballast {
        bal
    } else {
        error!("Ballast is required! See --help for futher info");
        return;
    };

    // Setting output file
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

    // Getting the Ballast
    if let Some(mut res) = parse_csv(ballast_file, BopType::Ballast) {
        // Getting the restrictor
        if let Some(rest_file) = args.restrictor {
            if let Some(rest_res) = parse_csv(rest_file, BopType::Restrictor) {
                // Merging the two lists
                for item in rest_res {
                    let mut index = 0;
                    for ent in res.iter() {
                        if item.track == ent.track && item.car_model == ent.car_model {
                            break;
                        }
                        index += 1;
                    }

                    if index >= res.len() {
                        // no match found, adding this as a new entry
                        res.push(item);
                    } else {
                        res[index].restrictor = item.restrictor;
                    }
                }
            } else {
                error!("Unable to parse restrictor csv, exiting...");
                return;
            }
        }

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
        error!("Unable to parse ballast csv, exiting...");
    }
}

#[derive()]
pub enum BopType {
    Ballast,
    Restrictor,
}

impl ToString for BopType {
    fn to_string(&self) -> String {
        match self {
            BopType::Ballast => "Ballast",
            BopType::Restrictor => "Restrictor",
        }
        .to_string()
    }
}

pub fn parse_csv(csv_file_path: String, file_type: BopType) -> Option<Vec<Entry>> {
    let path = PathBuf::from_str(&csv_file_path).ok()?;

    if !path.is_file() {
        error!("File does not exist!");
        return None;
    }

    info!("Loading {} file {}", file_type.to_string(), &csv_file_path);
    let file = fs::read_to_string(path).ok()?;

    let mut file = file.split("\n");
    let mut toprow = file.next()?.trim().split(",");
    toprow.next()?;

    let mut tracks: Vec<Option<String>> = vec![];
    for element in toprow {
		let element = element.trim();
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
            let mut row = car.trim().split(",");
            if let Some(model) = validate_car_model(row.next()) {
                // Reading the track entries
                let iter = zip(row, tracks.iter());
                for (element, track) in iter {
					let element = element.trim();
                    if let Some(track) = track {
                        // columns with bad headers still contain weights, we skip those but keep iterating to keep the order
                        entries.push(match file_type {
                            BopType::Ballast => create_ballast_entry(element, model, track),
                            BopType::Restrictor => create_restrictor_entry(element, model, track),
                        });
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

fn create_restrictor_entry(restrictor_string: &str, model: u32, track: &String) -> Entry {
    let restrictor_string = restrictor_string.trim();
    let restrictor_string = if restrictor_string.is_empty() {
        // this is done so we can error when the parse failed without erroring on empty
        "0"
    } else {
        restrictor_string
    };

    let car_name = get_car_name_from_id(model).unwrap_or(model.to_string());

    let rest = if let Ok(rest) = i32::from_str(restrictor_string) {
        if rest == 0 {
            None
        } else if rest < 0 {
            error!(
                "Restrictor for car {} at track {} was less then 0% ({}%), no Restrictor will be applied",
                car_name, track, rest
            );
            None
        } else if rest > 20 {
            error!(
                "Restrictor for car {} at track {} exceeded 20% ({}%), using 20%",
                car_name, track, rest
            );
            Some(20)
        } else {
            Some(rest)
        }
    } else {
        error!(
            "Failed to parse restrictor for car {} at track {}. String was {}. Skipping",
            car_name, track, restrictor_string
        );
        None
    };

    trace!(
        "car {} ({}) at {}: {}% Restrictor",
        car_name,
        model,
        track,
        rest.unwrap_or(0)
    );

    let entry = Entry {
        track: track.clone(),
        car_model: model,
        ballast_kg: None,
        restrictor: rest,
    };
    entry
}

pub fn validate_track(track_str: &str) -> Option<String> {
    let track_str = track_str
        .replace(" ", "_")
        .to_lowercase()
        .replace("bathurst", "mount_panorama")
        .replace("redbull_ring", "red_bull_ring")
        .replace("nordschleife", "nurburgring_24h");

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
		let text = text.trim();
        // Finding based on ID
        if let Some(id) = u32::from_str(text).ok() {
            if let Some(car_name) = get_car_name_from_id(id) {
                info!("Found car {} ({})", car_name, id);
                return Some(id);
            } else {
                error!("No car is known to have id {}", id)
            }
        }

        // We try to find the car based on the name, specifically we turn the text into tokens and then see if one carname contains all tokens
        let keywords: Vec<&str> = text
            .split(" ")
            .filter(|sample| !sample.trim().is_empty())
            .collect();
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

pub fn bop2csv(bop_json: String, output: Option<String>) -> Option<()> {
    let path = PathBuf::from_str(&bop_json).ok()?;

    if !path.is_file() {
        error!("bop.json File does not exist!");
        return None;
    }

    info!("Reading File...");

    let content = fs::read_to_string(bop_json).ok()?;
    let entries: BOP = serde_json::from_str(content.as_str()).ok()?;
    let entries = entries.entries;

    trace!("Finished Parsing json, converting to table...");

    // Parsing the entries into a table
    let mut table: HashMap<String, Vec<Option<Entry>>> = HashMap::new();
    let mut row_label: Vec<u32> = vec![];

    for item in entries {
        let mut row_index = 0;
        for i in row_label.iter() {
            if i == &item.car_model {
                break;
            }
            row_index += 1;
        }
        if row_index == row_label.len() {
            row_label.push(item.car_model);
        }

        if !table.contains_key(&item.track) {
            table.insert(item.track.clone(), Vec::<Option<Entry>>::with_capacity(row_label.len() + 1));
        }

        let column = table.get_mut(&item.track).expect("We insured it exists by creating one if they don't");
        while column.len() <= row_index {
            column.push(None);
        }

        column[row_index] = Some(item);
    }

    // Column headers
    let mut column_headers = Vec::<String>::with_capacity(23);
    for (c, _) in table.iter() {
        column_headers.push(c.to_string());
    }
    column_headers.sort();

    // Writing down the entries into Rows
    let mut rows = Vec::<Vec<Option<Entry>>>::with_capacity(row_label.len());
    let mut row_human_label = Vec::<String>::with_capacity(row_label.len());
    for row in row_label {
        row_human_label.push(get_car_name_from_id(row).unwrap_or(row.to_string()));

        let index = rows.len();
        let mut row_items = Vec::<Option<Entry>>::with_capacity(column_headers.len());

        for col in column_headers.iter() {
            let col = &table[col];
            if col.len() > index {
                row_items.push((&col[index]).clone());
            } else {
                row_items.push(None);
            }
        }

        rows.push(row_items);
    }
    
    trace!("Finished Tableizing");

    // Output path
    let (ballast_path, restrictor_path) = if let Some(target) = output {
        let mut target = PathBuf::from(target);

        if target.is_dir() {
            // User defined a folder... perfect, we just put both files into there
            let mut restrictor = target.clone();
            target.push("ballast.csv");
            restrictor.push("restrictor.csv");
            (target, restrictor)
        } else {
            // user defined a file, we will create a restrictor csv in the same folder
            let mut restrictor = target.clone();
            restrictor.set_file_name("restrictor.csv");
            (target, restrictor)
        }
    } else {
        (PathBuf::from("ballast.csv".to_string()), PathBuf::from("restrictor.csv".to_string()))
    };

    write_csv(&column_headers, &row_human_label, &rows, ballast_path, BopType::Ballast)?;
    write_csv(&column_headers, &row_human_label, &rows, restrictor_path, BopType::Restrictor)
}

fn write_csv(column_headers: &Vec<String>, row_human_label: &Vec<String>, rows: &Vec<Vec<Option<Entry>>>, path: PathBuf, file_type: BopType) -> Option<()> {
    trace!("Producing csv table for {}", file_type.to_string());
    // Write to file
    let mut output = String::new();
    for track in column_headers {
        output.push(',');
        output.push_str(track.as_str());
    }
    output.push('\n');

    let mut contains_anything = false;

    for (row_header, row) in zip(row_human_label, rows) {
        output.push_str(row_header.as_str());

        for item in row {
            output.push(',');

            output.push_str(if let Some(val) = item {
                match file_type {
                    BopType::Ballast =>
                        if let Some(ballast) = val.ballast_kg {
                            contains_anything = true;
                            ballast
                        } else {
                            0
                        },
                    BopType::Restrictor =>
                        if let Some(restrictor) = val.restrictor {
                            contains_anything = true;
                            restrictor
                        } else {
                            0
                        },
                }
                
            } else {
                0
            }.to_string().as_str());
        }

        output.push('\n');
    }

    if contains_anything {
        info!("Writing {}... ", file_type.to_string());
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
                return None;
            }
    
            fs::remove_file(&path).ok()?;
        }


        fs::write(path, output).ok()
    } else {
        trace!("Skipped writing {}, bop does not contain any changes to it", file_type.to_string());
        Some(())
    }
}
