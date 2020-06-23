use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use serde_json::Value;
use std::error::Error;
use std::io::{BufReader, Write};
use std::fs;
use dialoguer::Confirm;
use std::any::Any;

pub fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let u = serde_json::from_reader(reader)?;
    Ok(u)
}

pub fn set_json_value(hm: &mut HashMap<String, Value>, k: &str, v:Value) {
    if hm.get(k).unwrap().type_id() == v.type_id() {
        hm.insert(String::from(k), v);
        //Callback - value changed
    }
}

pub fn write(hm: &mut HashMap<String, Value>, filepath: &str){
    let data = serde_json::to_string_pretty(&hm).unwrap();
    let mut f = File::create(filepath).expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");
}

//[Shepmaster 2015](https://stackoverflow.com/questions/32384594/how-to-check-whether-a-path-exists)
pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn save_prompt(hm: &mut HashMap<String, Value>, backup_path:String) {
    if Confirm::new()
        .with_prompt("Are you sure you want save current changes to backup? Previous backup will be overwritten")
        .default(false)
        .interact()
        .unwrap() {
        write(hm,backup_path.as_str());
        println!("Current settings saved to backup");
    } else {
        println!("Save cancelled");
    }
}

pub fn revert_prompt(hm: &mut HashMap<String, Value>, backup_path:String, settings_folder: String) -> HashMap<String, Value>{
    if Confirm::new()
        .with_prompt("Are you sure you want to load backup? Any current changes will be deleted")
        .default(false).interact().unwrap() {
        //Load backup
        println!("Backup loaded");
        let backup = read_json_from_file(backup_path).unwrap();
        write(&mut backup.clone(),settings_folder.as_str());
        backup
    } else {
        println!("Revert cancelled");
        hm.clone()
    }
}




