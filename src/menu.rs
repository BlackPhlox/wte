use linked_hash_map::LinkedHashMap;
use serde_json::{Value, from_value};
use std::collections::HashMap;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use lazy_static::lazy_static;
use crate::{BACK,MENU_ITEM,SCHEMES,PROFILES,SETTINGS};

#[macro_use]
mod menu_macro {

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

    let index = setup_prompt(menu_type,(menu_stack.join(" > ")).to_string(), &finalsel, index_stuff);

    //println!("AF");

    //Check for setting type else menu process

    let selected = finalsel.get(index).unwrap();

    //let m : HashMap<String,Value> = from_value(wt.get("definitions").unwrap().clone()).unwrap();
    //println!("{:?}",m.keys());
    //println!("{:?}",wt);
    for key in wt.get("definitions") {
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
    let map = get_def_map(wt,"Globals");
    let properties : HashMap<String,Value> = from_value(map.get("properties").unwrap().clone()).unwrap();
    let p = properties.clone();
    //let setting_value_type : Vec<String> = from_value(map.get("type").unwrap().clone()).unwrap();

    println!("{:#?}",map_name(properties,keyword));
    p
    //let m : HashMap<String,Value> = from_value(wt.get("definitions").unwrap().clone()).unwrap();
    //from_value(m.get(keyword).unwrap().clone()).unwrap()
}

fn get_def_map(wt: &HashMap<String, Value>, keyword: &str) -> HashMap<String,Value>{
    let m : HashMap<String,Value> = from_value(wt.get("definitions").unwrap().clone()).unwrap();
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
        let a = hm.get("type").unwrap();
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
        .set_on_render(|i| println!("Selected {}",i))
        .with_prompt(prompt)
        .with_no_prompt_confirmation()
        .default(def)
        .items(&s)
        .interact()
        .unwrap()
}