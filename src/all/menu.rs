use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;
use serde_json::{Value, from_value, Number};
use dialoguer::{Select, MultiSelect, Confirm};
use dialoguer::theme::ColorfulTheme;
use lazy_static::lazy_static;

use crate::all::diff::{
    //Menu display
    MENU_SEPARATOR,
    //Menu items
    BACK, SAVE, REVERT, EDIT, EXIT, REMOVE,
    START_MENU, MENU_ITEM,
    GUI, DEFAULT_PROMPT,
    PROFILES, SCHEMES, SETTINGS,
    //Schema Definitions
    SCHEMA, DEF, REF, PROP, GLOBAL, TYPE,
    //Filepath Definitions
    CONFIG_PATH, CONFIG_FOLDER_PATH, SETTINGS_JSON, DEBUG_SCHEMA_PATH,
    BACKUP_EXTENSION,

    S_PROFILE, S_SCHEME
};


pub fn prompt_menu(
    menu_stack: &mut Vec<String>,
    index_stuff: &mut Vec<usize>,
    menu_relations: &mut LinkedHashMap<String,Vec<String>>,
    hm: &mut HashMap<String, Value>,
    wt: &HashMap<String, Value>)
    -> (String,usize) {
    let prompt_combo = menu_relations;
    let current = menu_stack.last().unwrap();//Latest pushed
    let fst = menu_stack.first().unwrap();

    let (selections,idx) = match prompt_combo.get( current) {
        Some(t) => (t,0),
        None if current == &String::from(BACK) => {(prompt_combo.get(fst).unwrap(), 0)}, //Back selected, get previous stack and pop current
        _ => {
            //Check prev
            let mut not_found = false;
            if menu_stack.len() > 2 {
                let prev_menu_string = menu_stack[menu_stack.len() - 2].clone();
                match prev_menu_string {
                    //Return the correct stuff
                    b if str_eq!(b,"Settings") => {
                        println!("Process as settings");
                        match menu_stack.last().unwrap() {
                            //Use "$ref" and "type" from schema
                            b if str_eq!(b,"defaultProfile") => {println!("GUID");}
                            _ => {not_found = true;}
                        }
                    },
                    b if str_eq!(b,"Profiles") => { println!("Process as profile");not_found = true;},
                    b if str_eq!(b,"Schemes") => { println!("Process as schemes");not_found = true;}
                    _ => {not_found = true;}
                }
            }
            //Fallen though because: not found
            if not_found {eprintln!("Error : Item not found");}

            //Pop from stack and go back to previous/last item in the stack
            menu_stack.pop();
            let old_menu_stack = menu_stack.last().unwrap();
            (prompt_combo.get(old_menu_stack).unwrap(),0)
        }
    };

    let mut sels = &mut selections.clone();
    let finalsel = &mut selections.clone();

    let menu_type = menu_stack.last().unwrap();

    //println!("{:#?}",menu_type);

    //Go though and find Settings Type Content Label and push all current settings to the menu stack
    match menu_type {
        b if str_eq!(b, SETTINGS) => {
            //Get only the changeable settings
            for key in hm.keys().filter(|&e| !(e == &SCHEMA || e == &PROFILES.to_lowercase() || e == &SCHEMES.to_lowercase())) {
                finalsel.insert(1, String::from(key));
            }
        },
        b if str_eq!(b, PROFILES) => {
            get_name_list(finalsel,hm,&PROFILES.to_lowercase(),true);
        },
        b if str_eq!(b, SCHEMES) => {
            get_name_list(finalsel,hm,&SCHEMES.to_lowercase(),true);
        },
        _ => {}
    }

    let index = setup_menu_prompt(menu_type, (menu_stack.join(MENU_SEPARATOR)).to_string(), &finalsel, index_stuff);

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
        println!("Menu item");
    } else {
        //Settings Item
        if str_eq!(menu_type, SETTINGS){
            //println!("{:?}",get_def_map(wt, "ProfileGuid").keys());

            //println!("{:?}",get_globals_map(wt, selected).keys());
            //println!("{:#?}", get_globals_map(wt, selected).keys());
            println!("Settings item");
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

fn get_name_list<'a>(finalsel: &'a mut Vec<String>, hm: &'a mut HashMap<String, Value>, keyword:&'a str, insert:bool) -> &'a mut Vec<String> {
    let val = hm.get(keyword).unwrap().clone();
    let arr : Vec<Value> = from_value(val).unwrap();
    for key in arr {
        let k : HashMap<String, Value> = from_value(key).unwrap();
        let k2 : String = from_value(k.get("name").unwrap().clone()).unwrap();
        if insert
        { finalsel.insert(1, k2); }
        else { finalsel.push(k2); }
    }
    finalsel
}

fn parse_def(props: HashMap<String, Value>, wt: &HashMap<String, Value>) -> (HashMap<String,Value>, String) {
    let a = props.get(REF).unwrap();
    //println!("{:#?}",a);
    let p : String = from_value(a.clone()).unwrap();
    //println!("{:#?}",p);
    let np = &p.replace("#/definitions/", "");
    //println!("{:#?}",np);
    (get_def_hashmap(wt), np.clone())
}

fn get_def_hashmap(wt: &HashMap<String, Value>) -> HashMap<String,Value>{
    from_value(wt.get(DEF).unwrap().clone()).unwrap()
}

fn get_def_map(wt: &HashMap<String, Value>, keyword: &str) -> HashMap<String,Value> {
    from_value(get_def_hashmap(wt).get(keyword).unwrap().clone()).unwrap()
}

//Usage:
//get_prop_map(wt,GLOBALS)
//get_prop_map(wt,PROFILE)
//get_prop_map(wt,SCHEME_LIST)
fn get_prop_obj(wt: &HashMap<String, Value>, main_def: &str) -> HashMap<String,Value>{
    let map = get_def_map(wt,main_def);
    let properties : HashMap<String,Value> = from_value(map.get(PROP).unwrap().clone()).unwrap();
    properties
}

fn get_globals_map(wt: &HashMap<String, Value>, keyword: &str) -> HashMap<String,Value>{
    let global_props = get_prop_obj(wt, GLOBAL);
    global_props
}

//Edit profile guids -> Create new or use existing profile
//Have a selection prompt with one selection called new, that prompts a guid editor
pub fn get_profile_guids(hm: &HashMap<String, Value>) -> Vec<(String,String)>{
    let val = hm.get(S_PROFILE).unwrap().clone();
    let arr : Vec<Value> = from_value(val).unwrap();

    let mut guids = vec![];
    for key in arr {
        let k : HashMap<String, Value> = from_value(key).unwrap();
        let k2 : String = from_value(k.get("name").unwrap().clone()).unwrap();
        let k3 : String = from_value(k.get("guid").unwrap().clone()).unwrap();
        guids.push((k2,k3));
    }
    guids
}

/// Get the properties and if they are currently being used in the settings.json
fn get_add_remove_properties(wt: &HashMap<String, Value>, hm: &mut HashMap<String, Value>, keyword: &str) -> Vec<(String,bool)> {
    let prop_map = get_prop_obj(wt, keyword);
    let key = match keyword {
        SCHEMES => Some(S_SCHEME),
        PROFILES => Some(S_PROFILE),
        _ => None
    };

    let newhm;
    if key.is_some() {
        let y = key.unwrap();
        //let hm2 : Vec<Value> = from_value(hm.get(y));

    } else {
        newhm = hm.clone();
    }

    let o = prop_map
        .iter()
        .map(|(a, _)|{
            let k = a.clone();
            let nk : Option<_> = SETTINGS_MAP.get(k.as_str());
            //Go to the correct dir for hm, "profiles" for PROFILE
            //And "schemes" for SCHEME
            //And nothing for GLOBAL/Settings
            if nk.is_some() {
                let v : &str = nk.unwrap();
                (v.to_string(),hm.contains_key(v))
            } else {
                let l = k.to_owned();
                let la = l.to_owned();
                (l,hm.contains_key(&la))
            }
        })
        .collect::<Vec<_>>();
    //println!("{:#?}",o);
    o
}

lazy_static!{
    /// Some definitions in WT is may need translation as the schema is still in development
    static ref SETTINGS_MAP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("requestedTheme", "theme");
        map.insert("theme", "requestedTheme");
        map
    };
}

/// Gets the list of types a property has, it does this by recursion if
/// It hits a $ref
fn get_keyword_editable_types(prop: HashMap<String,Value>, keyword: &str, wt: &HashMap<String, Value>) -> Vec<String>{
    let s = prop.get(keyword);
    if let Some(ref s) = s {

        let hm :HashMap<String,Value> = from_value(s.clone().clone()).unwrap();

        let a = hm.get(TYPE);
        if let Some(ref a) = a {
            let b = a.clone().clone();
            let o : Vec<String>;
            if b.is_array(){
                o = from_value(b).unwrap();
            } else {
                o = vec![from_value(b).unwrap()]
            }
            o
        } else {
            //If none, then it is because there is no type and must be using $ref
            //Get type using map_name recursively
            let (sh, keyword) = parse_def(hm.clone(), wt);
            get_keyword_editable_types(sh, keyword.as_str(), wt)
        }
    } else {
        //println!("{:#?}{:#?}",prop,keyword);
        get_keyword_editable_types(prop, SETTINGS_MAP.get(keyword).unwrap(), wt)
    }
}

/// Sets up the prompt for menu selection
fn setup_menu_prompt(_: &str, prompt: String, s: &[String], i: &mut Vec<usize>) -> usize {
    let def = if let Some(x) = i.pop() {x} else { 0usize };
    Select::with_theme(&ColorfulTheme::default())
        //.set_on_render(|i| println!("Selected {}",i))
        .no_confirmation()
        .with_prompt(prompt)
        .default(def)
        .items(&s)
        .interact()
        .unwrap()
}

/// Sets up a multi_select prompt when selecting add_remove menu item for profiles
pub fn setup_remove_profile_prompt(hm: &mut HashMap<String, Value>, keyword: &str){
    let mut a = vec![];
    let list = get_name_list(&mut a,hm,keyword,false);
    let mut defaults = (0..list.len()).map(|x| true).collect::<Vec<_>>();

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick profiles you want to remove")
        .paged(true)
        .no_confirmation()
        .items(list.as_ref())
        .defaults(defaults.as_ref())
        .interact()
        .unwrap();

    if selections.len() == list.len(){ return }

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(
            format!("Are you sure you want to remove {:#?}",
                    list.iter()
                        .enumerate()
                        .filter(|(i, a)|(!selections.contains(i)))
                        .map(|(_,b)|b).collect::<Vec<&String>>()
            )
        )
        .default(false)
        .interact().is_ok()
    {
        println!("Confirmed")
    }
    //print_stack_ln!(selections);
}

/// Sets up a multi_select prompt when selecting add_remove menu item for properties
pub fn setup_add_remove_prop_prompt(wt: &HashMap<String, Value>, hm: &mut HashMap<String, Value>, keyword: &str){
    let props = get_add_remove_properties(wt,hm,keyword);
    //props.iter().for_each(| (e,b) |{print!("{}({}) > ", e, b )});
    let (a,b) : (Vec<String>,Vec<bool>) = props.iter().map(|&(ref a, ref b)| (a.clone(), b.clone())).unzip();

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your properties you want to have")
        .paged(true)
        .no_confirmation()
        .items(a.as_ref())
        .defaults(b.as_ref())
        .interact()
        .unwrap();
}

fn from_slice(bytes: &[u8]) -> [u8; 32] {
    let mut array = [0; 32];
    let bytes = &bytes[..array.len()]; // panics if not enough data
    array.copy_from_slice(bytes);
    array
}