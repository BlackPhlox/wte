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
use dialoguer::theme::{Theme};
use std::{fmt, io};
use console::{Style, StyledObject, style, Term};

/* Term THEME */

pub(crate) struct WTETermThemeRenderer<'a> {
    term: &'a Term,
    theme: &'a dyn Theme,
    height: usize,
    prompt_height: usize,
    prompts_reset_height: bool,
}

impl<'a> WTETermThemeRenderer<'a> {
    pub fn new(term: &'a Term, theme: &'a dyn Theme) -> WTETermThemeRenderer<'a> {
        WTETermThemeRenderer {
            term,
            theme,
            height: 0,
            prompt_height: 0,
            prompts_reset_height: true,
        }
    }

    pub fn set_prompts_reset_height(&mut self, val: bool) {
        self.prompts_reset_height = val;
    }

    pub fn term(&self) -> &Term {
        self.term
    }

    pub fn add_line(&mut self) {
        self.height += 1;
    }

    fn write_formatted_str<
        F: FnOnce(&mut WTETermThemeRenderer, &mut dyn fmt::Write) -> fmt::Result,
    >(
        &mut self,
        f: F,
    ) -> io::Result<()> {
        let mut buf = String::new();
        f(self, &mut buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        self.height += buf.chars().filter(|&x| x == '\n').count();
        self.term.write_str(&buf)
    }

    fn write_formatted_line<
        F: FnOnce(&mut WTETermThemeRenderer, &mut dyn fmt::Write) -> fmt::Result,
    >(
        &mut self,
        f: F,
    ) -> io::Result<()> {
        let mut buf = String::new();
        f(self, &mut buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        self.height += buf.chars().filter(|&x| x == '\n').count() + 1;
        self.term.write_line(&buf)
    }

    fn write_formatted_prompt<
        F: FnOnce(&mut WTETermThemeRenderer, &mut dyn fmt::Write) -> fmt::Result,
    >(
        &mut self,
        f: F,
    ) -> io::Result<()> {
        self.write_formatted_line(f)?;
        if self.prompts_reset_height {
            self.prompt_height = self.height;
            self.height = 0;
        }
        Ok(())
    }

    pub fn error(&mut self, err: &str) -> io::Result<()> {
        self.write_formatted_line(|this, buf| this.theme.format_error(buf, err))
    }

    pub fn confirm_prompt(&mut self, prompt: &str, default: Option<bool>) -> io::Result<()> {
        self.write_formatted_str(|this, buf| this.theme.format_confirm_prompt(buf, prompt, default))
    }

    pub fn confirm_prompt_selection(&mut self, prompt: &str, sel: bool) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_confirm_prompt_selection(buf, prompt, sel)
        })
    }

    pub fn input_prompt(&mut self, prompt: &str, default: Option<&str>) -> io::Result<()> {
        self.write_formatted_str(|this, buf| this.theme.format_input_prompt(buf, prompt, default))
    }

    pub fn input_prompt_selection(&mut self, prompt: &str, sel: &str) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_input_prompt_selection(buf, prompt, sel)
        })
    }

    pub fn password_prompt(&mut self, prompt: &str) -> io::Result<()> {
        self.write_formatted_str(|this, buf| {
            write!(buf, "\r")?;
            this.theme.format_password_prompt(buf, prompt)
        })
    }

    pub fn password_prompt_selection(&mut self, prompt: &str) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_password_prompt_selection(buf, prompt)
        })
    }

    pub fn select_prompt(&mut self, prompt: &str) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| this.theme.format_select_prompt(buf, prompt))
    }

    pub fn select_prompt_selection(&mut self, prompt: &str, sel: &str) -> io::Result<()> {
        /*
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_select_prompt_selection(buf, prompt, sel)
        })*/
        self.write_formatted_line(|a,b|Ok(()))
    }

    pub fn select_prompt_item(&mut self, text: &str, active: bool) -> io::Result<()> {
        self.write_formatted_line(|this, buf| {
            this.theme.format_select_prompt_item(buf, text, active)
        })
    }

    pub fn multi_select_prompt(&mut self, prompt: &str) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| this.theme.format_multi_select_prompt(buf, prompt))
    }

    pub fn multi_select_prompt_selection(&mut self, prompt: &str, sel: &[&str]) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| {
            this.theme
                .format_multi_select_prompt_selection(buf, prompt, sel)
        })
    }

    pub fn multi_select_prompt_item(
        &mut self,
        text: &str,
        checked: bool,
        active: bool,
    ) -> io::Result<()> {
        self.write_formatted_line(|this, buf| {
            this.theme
                .format_multi_select_prompt_item(buf, text, checked, active)
        })
    }

    pub fn sort_prompt(&mut self, prompt: &str) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| this.theme.format_sort_prompt(buf, prompt))
    }

    pub fn sort_prompt_selection(&mut self, prompt: &str, sel: &[&str]) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_sort_prompt_selection(buf, prompt, sel)
        })
    }

    pub fn sort_prompt_item(&mut self, text: &str, picked: bool, active: bool) -> io::Result<()> {
        self.write_formatted_line(|this, buf| {
            this.theme
                .format_sort_prompt_item(buf, text, picked, active)
        })
    }

    pub fn clear(&mut self) -> io::Result<()> {
        self.term
            .clear_last_lines(self.height + self.prompt_height)?;
        self.height = 0;
        Ok(())
    }

    pub fn clear_preserve_prompt(&mut self, size_vec: &[usize]) -> io::Result<()> {
        let mut new_height = self.height;
        //Check each item size, increment on finding an overflow
        for size in size_vec {
            if *size > self.term.size().1 as usize {
                new_height += 1;
            }
        }
        self.term.clear_last_lines(new_height)?;
        self.height = 0;
        Ok(())
    }
}

/*
use crate::util::{read_json_from_file,write,path_exists,save_prompt,revert_prompt};
use crate::menu::{prompt_menu};
use crate::menu::menu_macro;
*/

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

//Menu display
pub const MENU_SEPARATOR: &str = " > ";

//Menu items:
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

//Types
pub const NULL: &str = "null";
pub const BOOL: &str = "boolean";
pub const INT: &str = "integer";
pub const NUM: &str = "number";
pub const STRING: &str = "string";
pub const COLOR: &str = "color";
pub const OBJECT: &str = "object";
pub const ARRAY: &str = "array";

//Schema Definitions
pub const DEF: &str = "definitions";
pub const PROP: &str = "properties";
pub const GLOBAL: &str = "Globals";
pub const TYPE: &str = "type";

//Filepath Definitions
pub const SETTINGS_JSON: &str = "settings.json";
pub const CONFIG_PATH: &str = "./src/config.json";
pub const DEBUG_SCHEMA_PATH: &str = "./src/wt_schema.json";
pub const CONFIG_FOLDER_PATH: &str = "settings_folder_path";
pub const BACKUP_EXTENSION: &str = ".backup";

/*
use crate::util::util::{path_exists,read_json_from_file,write,save_prompt,revert_prompt};
use wte::menu::{prompt_menu,path_exists,read_json_from_file,write,save_prompt,revert_prompt};
*/

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


//MENU

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
    let te = Term::buffered_stderr();
    //let wte_theme_render = WTETermThemeRenderer::new(&te.clone(),&ColorfulTheme::default());
    Select::with_theme(&ColorfulTheme::default())
        //.set_on_render(|i| println!("Selected {}",i))
        .with_prompt(prompt)
        .default(def)
        .items(&s)
        .interact_on(&te.clone())
        .unwrap()
}

//UTIL

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