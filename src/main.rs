use dialoguer::{Select, Confirm};
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
use core::slice::SliceIndex;
use dialoguer::theme::{Theme, ColorfulTheme};
use std::{fmt, io};
use lazy_static::lazy_static;
use console::{Style, StyledObject, style, Term, Emoji};

use wte::all::diff::{MENU_SEPARATOR, BACK, SAVE, REVERT, EDIT, EXIT, ADD_REMOVE, START_MENU, MENU_ITEM, GUI, DEFAULT_PROMPT, PROFILES, SCHEMES, SETTINGS, DEF, PROP, GLOBAL, TYPE, CONFIG_PATH, CONFIG_FOLDER_PATH, SETTINGS_JSON, DEBUG_SCHEMA_PATH, BACKUP_EXTENSION, PROFILE, S_PROFILE, S_SCHEME};

use wte::{str_eq,gen_menu_path,print_stack_ln};
use wte::all::util::{read_json_from_file, write, path_exists, save_prompt, revert_prompt};
use wte::all::menu::{prompt_menu, setup_add_remove_prop_prompt, setup_remove_profile_prompt};
use wte::all::gui::start_gui_server;

fn main() {
    //Load and set configs
    let config = read_json_from_file(CONFIG_PATH).unwrap();
    let settings_folder : String = from_value(config.get(CONFIG_FOLDER_PATH).unwrap().clone()).unwrap();
    let settings_path: String = settings_folder + SETTINGS_JSON;

    //Load Schema
    let wt = read_json_from_file(DEBUG_SCHEMA_PATH).unwrap();

    //Load wt settings
    let mut hm = read_json_from_file(settings_path.clone()).unwrap();

    //Do backup
    let backup_path = settings_path.clone() + BACKUP_EXTENSION;
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
        START_MENU; EDIT, REVERT,   SAVE,     EXIT;
        EDIT      ; SETTINGS, PROFILES, SCHEMES, GUI, BACK;
        SETTINGS  ; ADD_REMOVE, BACK;
        PROFILES  ; ADD_REMOVE, BACK;
        SCHEMES   ; ADD_REMOVE, BACK
    ];

    //Set start position
    let mut current_menu_stack = vec![(String::from(START_MENU))];
    let mut current_selection_index_stack = vec![0usize];

    //Start menu-navigation loop
    loop {
        //[Debug] Show saved index stack for each selection
        //print_stack_ln!(current_selection_index_stack);

        let (a,i) = prompt_menu(&mut current_menu_stack,&mut current_selection_index_stack, &mut l, &mut hm, &wt);
        current_selection_index_stack.push(i);
        match a {
            b if str_eq!(b, GUI) => { start_gui_server(); continue; },
            b if str_eq!(b, SAVE) => { save_prompt(&mut hm, backup_path.clone()); continue; },
            b if str_eq!(b, REVERT) => { hm = revert_prompt(&mut hm, backup_path.clone(), settings_path.clone()); continue; },
            b if str_eq!(b, BACK) => { current_menu_stack.pop(); current_selection_index_stack.pop(); continue; },
            b if str_eq!(b, ADD_REMOVE) => { //Use multi select in combination with default of the schema
                wt_setting_types(&mut current_menu_stack,
                                 || {println!("{:#?}", "ARSE"); setup_add_remove_prop_prompt(&wt, &mut hm.clone(), GLOBAL)},
                                 || {println!("{:#?}", "ARPR"); setup_remove_profile_prompt(&mut hm.clone(), S_PROFILE) /*setup_add_remove_prop_prompt(&wt, &mut hm.clone(), PROFILE)*/},
                                 || {println!("{:#?}", "ARSC"); setup_remove_profile_prompt(&mut hm.clone(), S_SCHEME)}
                ); continue; },
            b if str_eq!(b, EXIT) => { break; },
            _ => {
                current_menu_stack.push(a);
                current_selection_index_stack.push(i);
                continue;
            }
        };
    }
}

fn wt_setting_types<F1,F2,F3>(menu_stack: &mut Vec<String>, settings_fun: F1, profiles_fun: F2, schemes_fun: F3)
    where F1:FnOnce(),F2:FnOnce(),F3:FnOnce() {
        match menu_stack.last().unwrap() {
            b if str_eq!(b, SETTINGS) => { settings_fun() },
            b if str_eq!(b, PROFILES) => { profiles_fun() },
            b if str_eq!(b, SCHEMES)  => { schemes_fun()  }
            _ => { menu_stack.pop(); }
        };
}