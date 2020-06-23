use linked_hash_map::LinkedHashMap;
use serde_json::{Value, from_value};
use std::collections::HashMap;
use dialoguer::{Select, Confirm};
use dialoguer::theme::ColorfulTheme;
use lazy_static::lazy_static;
use crate::main::{BACK,MENU_ITEM,SCHEMES,PROFILES,SETTINGS};
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, Write};
use std::any::Any;
use std::fs;
use std::error::Error;

#[macro_use]
pub mod menu_macro {


}
