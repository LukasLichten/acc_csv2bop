use std::{fs, path::PathBuf};

fn clean_up() {
    if PathBuf::from("test").exists() {
        fs::remove_dir_all("test").expect("Clean up operation failed");
    }
}

#[test]
fn simple_reparse_check() {
    // reparse check parses the sample-bop.json to a csv, then reparses into a bop.json

    // Targets variables
    let sample = "samples/sample-bop.json".to_string();

    let test_csv_string = "test/test.csv".to_string();

    // Check if sample is present
    assert!(PathBuf::from(&sample).exists(), "Test Setup Failed: /samples/sample-bop.json is missing");
    let sample_text = fs::read_to_string(sample.as_str()).expect("Test Setup Failed: Failed to read sample-bop.json");
    let sample_bop: crate::BOP = serde_json::from_str(sample_text.as_str()).expect("Test Setup Failed: Could not parse sample-bop.json into Entries");
    let sample_entries = sample_bop.entries;

    // Setup
    clean_up();
    fs::create_dir("test").expect("Setup up of the test failed");
    let test_csv_option = Some(test_csv_string.clone());

    // Running Functions to test
    crate::bop2csv(sample.clone(), test_csv_option).expect("Failed to parse bop.json to csv");

    let output = crate::parse_csv(test_csv_string, crate::BopType::Ballast).expect("Failed to parse csv to entries");

    clean_up();

    
    // Removing entries with no bop adjustment, like we do in main
    let mut entries = Vec::<crate::Entry>::with_capacity(output.len());
    for item in output {
        if item.ballast_kg.is_some() || item.restrictor.is_some() {
            entries.push(item);
        }
    }

    // Parsing output to json for completness
    serde_json::to_string_pretty(&crate::BOP { entries: entries.clone() }).expect("Failed to parse output back into json"); // We can't simply compare json output due to the order changing

    // Comparing the output and the entries
    for entry in sample_entries {
        assert!(entries.contains(&entry), "Failed to find sample-bop entry {} at {} with {}kg and {}% in output", entry.car_model.clone(), entry.track.clone(), entry.ballast_kg.unwrap_or(0), entry.restrictor.unwrap_or(0));

        let mut index = 0;
        for item in entries.iter() {
            if &entry == item {
                break;
            }
            index += 1;
        }
        
        entries.remove(index);
    }

    assert!(entries.is_empty(), "There were more entries then there should be");

}