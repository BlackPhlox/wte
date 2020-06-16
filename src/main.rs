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
use lazy_static::lazy_static;
use core::slice::SliceIndex;
use crate::util::path_exists;

mod util;
#[macro_use]
mod menu;

//Constants:
pub const BACK: &str = "Back";
pub const SAVE: &str = "Save";
pub const REVERT: &str = "Revert";
pub const EDIT: &str = "Edit";
pub const EXIT: &str = "Exit";
pub const ADD_REMOVE: &str = "Add/Remove";
//static ADD: &str = "Add";
//static REMOVE: &str = "Remove";
pub const START_MENU: &str = "Main menu";
pub const GUI: &str = "GUI";
pub const DEFAULT_PROMPT: &str = "What do you want to do?";
pub const SETTINGS: &str = "Settings";
pub const PROFILES: &str = "Profiles";
pub const SCHEMES: &str = "Schemes";
pub const MENU_ITEM:  &'static [&str] = &[BACK,SAVE,REVERT,EDIT,EXIT,GUI,ADD_REMOVE/*ADD,REMOVE*/,START_MENU,SETTINGS,PROFILES,SCHEMES];

fn main() {
    //Load and set configs
    let config = util::read_json_from_file("./src/config.json").unwrap();
    let settings_folder : String = from_value(config.get("settings_folder_path").unwrap().clone()).unwrap();
    let settings_path: String = (settings_folder +"settings.json");

    //Load Schema
    let wt = util::read_json_from_file("./src/wt_schema.json").unwrap();

    //Load wt settings
    let mut hm = util::read_json_from_file(settings_path.clone()).unwrap();

    //Do backup
    let backup_path = settings_path.clone()+".backup";
    if !util::path_exists(backup_path.clone().as_str()) {
        util::write(&mut hm,backup_path.clone().as_str());
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
        START_MENU; EDIT, REVERT,   SAVE,     EXIT;
        EDIT      ; SETTINGS, PROFILES, SCHEMES, GUI, BACK;
        SETTINGS  ; ADD_REMOVE, BACK;
        PROFILES  ; ADD_REMOVE, BACK;
        SCHEMES   ; ADD_REMOVE, BACK
    ];

    //Set start position
    let mut current_menu_stack = vec![(String::from(START_MENU),0)];
    let mut current_selection_index_stack = vec![0usize];

    //Start menu-navigation loop
    loop {
        current_selection_index_stack.iter().for_each(|(a)|print!(" {:#} >",a));
        println!();
        //current_menu_stack.iter().for_each(|(m,i)|{print!("{menu}({index})",menu = m, index = i);println!()});
        let (a,i) = menu::prompt_menu(&mut current_menu_stack,&mut current_selection_index_stack, &mut l, &mut hm, &wt);
        current_selection_index_stack.push(i);
        let mut cms : Vec<String> = current_menu_stack.iter().map(|(a,b)| {a.clone()} ).collect::<Vec<String>>();
        match a {
            b if str_eq!(b, GUI) => { start_gui(); continue; },
            b if str_eq!(b, SAVE) => { util::save_prompt(&mut hm, backup_path.clone()); continue; },
            b if str_eq!(b, REVERT) => { hm = util::revert_prompt(&mut hm, backup_path.clone(), settings_path.clone()); continue; },
            b if str_eq!(b, BACK) => { current_menu_stack.pop(); current_selection_index_stack.pop(); continue; },
            b if str_eq!(b, ADD_REMOVE) => { //Use multi select in combination with default of the schema
                m_setting_types( &mut cms,
                                 || println!("{:#?}", "ARSE"),
                                 || println!("{:#?}", "ARPR"),
                                 || println!("{:#?}", "ARSC")
                ); continue; },
            b if str_eq!(b, EXIT) => { break; },
            _ => { current_menu_stack.push((a,i)); continue; }
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

fn start_gui(){
    println!("Starting GUI Server");

    println!("Stopping GUI Server");
}