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
pub const SCHEMA: &str = "$schema";
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

#[macro_export]
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

#[macro_export]
macro_rules! print_stack_ln {
        ($a: expr) => {
            $a.clone().iter_mut().for_each(| e | { print!("{} > ", e ) });
            println!();
        }
}

#[macro_export]
macro_rules! str_eq {
        ($a: expr,$b: expr) => {
            $a == $b
        }
}