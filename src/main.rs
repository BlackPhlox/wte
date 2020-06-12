use dialoguer::{theme::ColorfulTheme, Select, Confirm};
use linked_hash_map::LinkedHashMap;
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use serde_json::{Value, from_value};
use serde_json::json;
use std::collections::HashMap;
use serde_json::value::Value::Number;
use std::any::Any;
use std::fs;
use core::borrow::Borrow;
use std::convert::TryInto;

//Constants:
static BACK: &str = "Back";
static SAVE: &str = "Save";
static REVERT: &str = "Revert";
static EDIT: &str = "Edit";
static EXIT: &str = "Exit";
static ADD: &str = "Add";
static REMOVE: &str = "Remove";
static START_MENU: &str = "Main menu";
static DEFAULT_PROMPT: &str = "What do you want to do?";
static SETTINGS: &str = "Settings";
static PROFILES: &str = "Profiles";
static SCHEMES: &str = "Schemes";

macro_rules! gen_menu_path {
    ($($menu: expr;$($access:expr),*);+) => {{
        let mut map = ::linked_hash_map::LinkedHashMap::new();
            $(
                let mut v = vec![];
                $(v.push(String::from($access));)*
                map.insert(String::from($menu), v);
            )*
        map
    }}
}

macro_rules! print_stack_ln {
    ($a: expr) => {
        $a.clone().iter_mut().for_each(| e | { print!("{} > ", e ) });
        println!();
    }
}

macro_rules! str_eq {
    ($a: expr,$b: expr) => {
        $a == $b
    }
}

fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let u = serde_json::from_reader(reader)?;
    Ok(u)
}

fn set_json_value(hm: &mut HashMap<String, Value>, k: &str, v:Value) {
    if hm.get(k).unwrap().type_id() == v.type_id() {
        hm.insert(String::from(k), v);
        //Callback - value changed
    }
}

fn write(hm: &mut HashMap<String, Value>, filepath: &str){
    let data = serde_json::to_string_pretty(&hm).unwrap();
    let mut f = File::create(filepath).expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");
}

//[Shepmaster 2015](https://stackoverflow.com/questions/32384594/how-to-check-whether-a-path-exists)
pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn save_prompt(hm: &mut HashMap<String, Value>, backup_path:String) {
    if Confirm::new().with_prompt("Are you sure you want save current changes to backup? Previous backup will be overwritten").interact().unwrap() {
        write(hm,backup_path.as_str());
        println!("Current settings saved to backup");
    } else {
        println!("Save cancelled");
    }
}

fn revert_prompt(hm: &mut HashMap<String, Value>, backup_path:String, settings_folder: String) -> HashMap<String, Value>{
    if Confirm::new().with_prompt("Are you sure you want to load backup? Any current changes will be deleted").interact().unwrap() {
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

fn main() {
    //Load and set configs
    let config = read_json_from_file("./src/config.json").unwrap();
    let settings_folder : String = from_value(config.get("settings_folder_path").unwrap().clone()).unwrap();
    let settings_path: String = (settings_folder +"settings.json");

    //Load Schema
    //let schema = read_json_from_file("./src/wt_schema.json").unwrap();

    //Load wt settings
    let mut hm = read_json_from_file(settings_path.clone()).unwrap();

    //Do backup
    let backup_path = settings_path.clone()+".backup";
    if !path_exists(backup_path.clone().as_str()) {
        write(&mut hm,backup_path.clone().as_str());
    }

    /*
        let key = String::from("initialRows");
        let val = hm.get(&key).unwrap();
        set_json_value(&mut hm, &key, json!(2));
        println!("{:#?}", hm.get(&key));
        println!("{:#?}", config.keys());
        write(&mut hm,settings_folder.clone().as_str());
    */

    //Setup menu
    let mut l = gen_menu_path![
        START_MENU; EDIT, REVERT,   SAVE,             EXIT;
        EDIT      ; BACK, SETTINGS, PROFILES,         SCHEMES;
        SETTINGS  ; ADD,  REMOVE, BACK;
        PROFILES  ; ADD,  REMOVE, BACK;
        SCHEMES   ; ADD,  REMOVE,  BACK
    ];

    //Set start position
    let mut current_menu_stack = vec![String::from(START_MENU)];

    //Start menu-navigation loop
    loop {
        let a = prompt_menu(&mut current_menu_stack, &mut l, &mut hm);
        match a {
            b if str_eq!(b, "Program1") => { program1(); continue; },
            b if str_eq!(b, SAVE) => { save_prompt(&mut hm, backup_path.clone()); continue; },
            b if str_eq!(b, REVERT) => { hm = revert_prompt(&mut hm, backup_path.clone(), settings_path.clone()); continue; },
            b if str_eq!(b, BACK) => { current_menu_stack.pop(); continue; },
            b if str_eq!(b, ADD) => {
                m_setting_types(&mut current_menu_stack,
                                || println!("{:#?}", "ASE"),
                                || println!("{:#?}", "APR"),
                                || println!("{:#?}", "ASC")
                ); continue; },
            b if str_eq!(b, REMOVE) => {
                m_setting_types(&mut current_menu_stack,
                                || println!("{:#?}", "RSE"),
                                || println!("{:#?}", "RPR"),
                                || println!("{:#?}", "RSC")
                ); continue; },
            b if str_eq!(b, EXIT) => { break; },
            _ => { current_menu_stack.push(a); continue; }
        };
    }
}

fn m_setting_types<F1,F2,F3>(menu_stack: &mut Vec<String>, settings_fun: F1, profiles_fun: F2, schemes_fun: F3)
    where F1:FnOnce(),F2:FnOnce(),F3:FnOnce() {
        match menu_stack.last().unwrap() {
            b if str_eq!(b, SETTINGS) => { settings_fun() },
            b if str_eq!(b, PROFILES) => { profiles_fun() },
            b if str_eq!(b, SCHEMES)  => { schemes_fun()  }
            _ => { menu_stack.pop(); }
        };
}

fn prompt_menu(menu_stack: &mut Vec<String>, menu_relations: &mut LinkedHashMap<String,Vec<String>>, hm: &mut HashMap<String, Value>) -> String {
    let prompt_combo = menu_relations;
    let current = menu_stack.last().unwrap();//Latest pushed
    let selections = match prompt_combo.get( current) {
        Some(t) => t,
        None if current == &String::from(BACK) => prompt_combo.get(menu_stack.first().unwrap()).unwrap(), //Back selected, get previous stack and pop current
        _ => {
            //Not found, pop from stack and go back to previous/last item in the stack
            eprintln!("Error : Item not found");
            menu_stack.pop();
            prompt_combo.get(menu_stack.last().unwrap()).unwrap()
        }
    };

    let mut sels = &mut selections.clone();
    let mut finalsel = &mut selections.clone();

    //Go though and find Settings Type Content Label and push all current settings to the menu stack
    match menu_stack.last().unwrap() {
        b if str_eq!(b, SETTINGS) => {
            for key in hm.keys().filter(|&e| !(e == "$schema" || e == &PROFILES.to_lowercase() || e == &SCHEMES.to_lowercase())) {
                finalsel.insert(2, String::from(key));
            }
        },
        b if str_eq!(b, PROFILES) => {
            let val = hm.get(&PROFILES.to_lowercase()).unwrap().clone();
            let arr : Vec<Value> = from_value(val).unwrap();
            for key in arr {
                let k : HashMap<String, Value> = from_value(key).unwrap();
                let k2 : String = from_value(k.get("name").unwrap().clone()).unwrap();
                finalsel.insert(2, k2);
            }
        },
        b if str_eq!(b, SCHEMES) => {
            let val = hm.get(&SCHEMES.to_lowercase()).unwrap().clone();
            let arr : Vec<Value> = from_value(val).unwrap();
            for key in arr {
                let k : HashMap<String, Value> = from_value(key).unwrap();
                let k2 : String = from_value(k.get("name").unwrap().clone()).unwrap();
                finalsel.insert(2, k2);
            }
        },
        _ => {}
    }

    let index = setup_prompt(menu_stack.last().unwrap(),(menu_stack.join(" > ")).to_string(), &finalsel);

    //Check for setting type else menu process

    let selected = &finalsel.get(index);
    selected.unwrap().clone()
}

fn setup_prompt(_: &str, prompt: String, s: &[String]) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .with_no_prompt_confirmation()
        .default(0)
        .items(&s)
        .interact()
        .unwrap()
}

fn program1() {
    println!("Program1 has completed");
}

