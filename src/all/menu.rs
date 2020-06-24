use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;
use serde_json::{Value, from_value};
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use lazy_static::lazy_static;

use crate::all::diff::{
    //Menu display
    MENU_SEPARATOR,
    //Menu items
    BACK, SAVE, REVERT, EDIT, EXIT, ADD_REMOVE,
    START_MENU, MENU_ITEM,
    GUI, DEFAULT_PROMPT,
    PROFILES, SCHEMES, SETTINGS,
    //Schema Definitions
    SCHEMA, DEF, PROP, GLOBAL, TYPE,
    //Filepath Definitions
    CONFIG_PATH, CONFIG_FOLDER_PATH, SETTINGS_JSON, DEBUG_SCHEMA_PATH,
    BACKUP_EXTENSION,
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
            //Not found, pop from stack and go back to previous/last item in the stack
            eprintln!("Error : Item not found");
            menu_stack.pop();
            let old_menu_stack = menu_stack.last().unwrap();
            (prompt_combo.get(old_menu_stack).unwrap(),0)
        }
    };

    let mut sels = &mut selections.clone();
    let finalsel = &mut selections.clone();

    let menu_type = menu_stack.last().unwrap();

    //Go though and find Settings Type Content Label and push all current settings to the menu stack
    match menu_type {
        b if str_eq!(b, SETTINGS) => {
            //Get only the changeable settings
            for key in hm.keys().filter(|&e| !(e == &SCHEMA || e == &PROFILES.to_lowercase() || e == &SCHEMES.to_lowercase())) {
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

    //Get missing properties for Add_Remove

    let index = setup_prompt(menu_type,(menu_stack.join(MENU_SEPARATOR)).to_string(), &finalsel, index_stuff);

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


    println!("{:#?}", get_keyword_editable_types(properties, keyword, wt));

    p
    //let m : HashMap<String,Value> = from_value(wt.get("definitions").unwrap().clone()).unwrap();
    //from_value(m.get(keyword).unwrap().clone()).unwrap()
}

fn parse_def(props: HashMap<String, Value>, wt: &HashMap<String, Value>) -> (HashMap<String,Value>, String) {
    let a = props.get("$ref").unwrap();
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

lazy_static!{
    static ref SETTINGS_MAP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("requestedTheme", "theme");
        map
    };
}

fn get_keyword_editable_types(prop: HashMap<String,Value>, keyword: &str, wt: &HashMap<String, Value>) -> Vec<String>{
    let s = prop.get(keyword);
    println!("{:#?}",s);
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
        get_keyword_editable_types(prop, SETTINGS_MAP.get(keyword).unwrap(), wt)
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