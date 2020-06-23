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

use wte::all::diff::{
    //Menu display
    MENU_SEPARATOR,
    //Menu items
    BACK, SAVE, REVERT, EDIT, EXIT, ADD_REMOVE,
    START_MENU, MENU_ITEM,
    GUI, DEFAULT_PROMPT,
    PROFILES, SCHEMES, SETTINGS,
    //Schema Definitions
    DEF, PROP, GLOBAL, TYPE,
    //Filepath Definitions
    CONFIG_PATH, CONFIG_FOLDER_PATH, SETTINGS_JSON, DEBUG_SCHEMA_PATH,
    BACKUP_EXTENSION,
};

use wte::{str_eq,gen_menu_path,print_stack_ln};
use wte::all::util::{read_json_from_file, write, path_exists, save_prompt, revert_prompt};

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
    let mut current_menu_stack = vec![(String::from(START_MENU),0)];
    let mut current_selection_index_stack = vec![0usize];

    //Start menu-navigation loop
    loop {
        //current_selection_index_stack.iter().for_each(|(a)|print!(" {:#} >",a));
        //println!();
        //current_menu_stack.iter().for_each(|(m,i)|{print!("{menu}({index})",menu = m, index = i);println!()});
        let (a,i) = prompt_menu(&mut current_menu_stack,&mut current_selection_index_stack, &mut l, &mut hm, &wt);
        current_selection_index_stack.push(i);
        let mut cms : Vec<String> = current_menu_stack.iter().map(|(a,b)| {a.clone()} ).collect::<Vec<String>>();
        match a {
            b if str_eq!(b, GUI) => { start_gui(); continue; },
            b if str_eq!(b, SAVE) => { save_prompt(&mut hm, backup_path.clone()); continue; },
            b if str_eq!(b, REVERT) => { hm = revert_prompt(&mut hm, backup_path.clone(), settings_path.clone()); continue; },
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

pub fn prompt_menu(
    comb_stack: &mut Vec<(String,usize)>,
    index_stuff: &mut Vec<usize>,
    menu_relations: &mut LinkedHashMap<String,Vec<String>>,
    hm: &mut HashMap<String, Value>,
    wt: &HashMap<String, Value>)
    -> (String,usize) {
    let prompt_combo = menu_relations;
    let mut menu_stack : Vec<String> = comb_stack.iter().map(|(a,b)| {a.clone()} ).collect::<Vec<String>>();
    let (current, i) = comb_stack.last().unwrap();//Latest pushed
    let (fst, fst_i) = comb_stack.first().unwrap();

    /*
    println!("{:#}", current);
    comb_stack.iter().for_each(|(m,i)|{print!("{menu}({index})",menu = m, index = i);println!()});
    let (_,l) = comb_stack.last().unwrap();
    println!("L:{:#}",l);
    let (_,current_l) = &mut cl.pop().unwrap();
    println!("{:#}", prev_l);
    */

    let (selections,idx) = match prompt_combo.get( current) {
        Some(t) => (t,0),
        None if current == &String::from(BACK) => {(prompt_combo.get(fst).unwrap(), 0)}, //Back selected, get previous stack and pop current
        _ => {
            //Not found, pop from stack and go back to previous/last item in the stack
            eprintln!("Error : Item not found");
            menu_stack.pop();
            let old_menu_stack = menu_stack.last().unwrap();
            (prompt_combo.get(old_menu_stack).unwrap(),0)
        }
    };

    let mut sels = &mut selections.clone();
    let mut finalsel = &mut selections.clone();

    let mut menu_type = menu_stack.last().unwrap();

    //Go though and find Settings Type Content Label and push all current settings to the menu stack
    match menu_type {
        b if str_eq!(b, SETTINGS) => {
            for key in hm.keys().filter(|&e| !(e == "$schema" || e == &PROFILES.to_lowercase() || e == &SCHEMES.to_lowercase())) {
                finalsel.insert(1, String::from(key));
            }
        },
        b if str_eq!(b, PROFILES) => {
            let val = hm.get(&PROFILES.to_lowercase()).unwrap().clone();
            let arr : Vec<Value> = from_value(val).unwrap();
            for key in arr {
                let k : HashMap<String, Value> = from_value(key).unwrap();
                let k2 : String = from_value(k.get("name").unwrap().clone()).unwrap();
                finalsel.insert(1, k2);
            }
        },
        b if str_eq!(b, SCHEMES) => {
            let val = hm.get(&SCHEMES.to_lowercase()).unwrap().clone();
            let arr : Vec<Value> = from_value(val).unwrap();
            for key in arr {
                let k : HashMap<String, Value> = from_value(key).unwrap();
                let k2 : String = from_value(k.get("name").unwrap().clone()).unwrap();
                finalsel.insert(1, k2);
            }
        },
        _ => {}
    }

    //comb_stack.iter().for_each(|(a,b)|print!(" {:#} >",b));
    //println!();

    //println!("BF");

    let index = setup_prompt(menu_type,(menu_stack.join(MENU_SEPARATOR)).to_string(), &finalsel, index_stuff);

    //println!("AF");

    //Check for setting type else menu process

    let selected = finalsel.get(index).unwrap();

    //let m : HashMap<String,Value> = from_value(wt.get("definitions").unwrap().clone()).unwrap();
    //println!("{:?}",m.keys());
    //println!("{:?}",wt);
    for key in wt.get(DEF) {
        //println!("{:?}",key);
    }

    if MENU_ITEM.contains(&&***&selected){
        //Menu Item

    } else {
        //Settings Item
        if str_eq!(menu_type, SETTINGS){
            //println!("{:?}",get_def_map(wt, "ProfileGuid").keys());

            let v = get_globals_map(wt, selected).keys();
            //println!("{:?}",get_globals_map(wt, selected).keys());
        }

    }

    /* Get:
        "default": "{}",
        "pattern": "^\\{[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}\\}$",
        "type": "string"
    */

    //Lookup key in wt_scheme

    (selected.clone(),index)
}

fn get_globals_map(wt: &HashMap<String, Value>, keyword: &str) -> HashMap<String,Value>{
    let map = get_def_map(wt,GLOBAL);
    let properties : HashMap<String,Value> = from_value(map.get(PROP).unwrap().clone()).unwrap();
    let p = properties.clone();
    //let setting_value_type : Vec<String> = from_value(map.get("type").unwrap().clone()).unwrap();

    println!("{:#?}",map_name(properties,keyword));
    p
    //let m : HashMap<String,Value> = from_value(wt.get("definitions").unwrap().clone()).unwrap();
    //from_value(m.get(keyword).unwrap().clone()).unwrap()
}

fn get_def_map(wt: &HashMap<String, Value>, keyword: &str) -> HashMap<String,Value>{
    let m : HashMap<String,Value> = from_value(wt.get(DEF).unwrap().clone()).unwrap();
    from_value(m.get(keyword).unwrap().clone()).unwrap()
}

lazy_static!{
    static ref SETTINGS_MAP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("requestedTheme", "theme");
        map
    };
}

fn map_name(prop: HashMap<String,Value>, keyword: &str) -> Vec<String>{
    //Become let s : HashMap<String,Value>= from_value(prop.get(keyword).unwrap().clone()).unwrap();
    let s = prop.get(keyword);
    if let Some(ref s) = s {
        let hm :HashMap<String,Value> = from_value(s.clone().clone()).unwrap();
        let a = hm.get(TYPE).unwrap();
        let b = a.clone();
        let o : Vec<String>;
        if b.is_array(){
            o = from_value(b).unwrap();
        } else {
            o = vec![from_value(b).unwrap()]
        }
        o
    } else {
        vec![SETTINGS_MAP.get(keyword).unwrap().to_string()]
    }
}

fn setup_prompt(_: &str, prompt: String, s: &[String], i: &mut Vec<usize>) -> usize {
    let def = if let Some(x) = i.pop() {x} else { 0usize };
    Select::with_theme(&ColorfulTheme::default())
        //.set_on_render(|i| println!("Selected {}",i))
        .with_prompt(prompt)
        .default(def)
        .items(&s)
        .interact()
        .unwrap()
}
