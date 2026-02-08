// netoe1: This file is responsable to implement a config file for JSON.


// netoe1: Libraries and external dependencies.
use serde::Deserialize; 
use std::fs::File;
use std::io::BufReader;

// netoe1: Our dependencies
// Importing Methods:
use crate::config::{set_pen_threshold, set_pen_strength};

const CONFIG_FILE: &str = "cfg.json";

// netoe1: Check config.rs to make sense. (I'll create a README file later...)
#[derive(Deserialize, Debug)]
struct ConfigJSONStruct {
    pen_threshold: i32,          
    pen_strength_scaling: i32        
}

// netoe1: Function to read cfg json file to
fn readcfgFile(path: &str) -> ConfigJSONStruct {
    // netoe1: Read from file, using the actual structure defined above.

    let file = File::open(path).expect("mx002-err: Cannot find or open JSON config file.");
    let reader = BufReader::new(file);
    let config: ConfigJSONStruct = serde_json::from_reader(reader)
        .expect("mx002-err: Error to parse JSON file. Check Structure first! ");

    return config;
}

// netoe1: Set values getting from JSON's to load in driver's!
fn setValuesToMemory(json_parsed: ConfigJSONStruct){
    // netoe1: In config.rs file, we have a simple interface that connects these values
    // to global constants, that I'll be loaded later by the driver.

    // These functions are:
    // pub fn set_pen_threshold(value: i32) 
    // pub fn set_pen_strength(value: i32)

    set_pen_strength(json_parsed.pen_strength_scaling);
    set_pen_threshold(json_parsed.pen_threshold);
}

