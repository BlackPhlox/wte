use dialoguer::{Select, Confirm, Input, Editor};
use dialoguer::theme::ColorfulTheme;
use serde_json::{Value, from_value};
use serde_json::json;
use std::collections::HashMap;
use css_color_parser::Color as CssColor;
use core::fmt::Pointer;
use colors_transform::{Rgb, Color};


/*
use wte::all::diff::{
    //WT Types
    NULL,BOOL,INT,NUM,STRING,COLOR,OBJECT,ARRAY
};
*/

//potential : https://github.com/Marwes/schemafy

pub fn set_value_types(s_arr: Vec<&str>, property_name: &str, current_value: Option<Value>) -> String{
    let properties : HashMap<String,Value> = HashMap::new();
    if s_arr.len() > 1 {
        //New select prompt
        //Choose of which to use

        //check for oneOf
        //if true, select which to use
        if let Some(x) = properties.get("oneOf"){
            //Is array
            //Contains objects with a "type" and maybe enum if type is string
        }

        let index =
            Select::with_theme(&ColorfulTheme::default())
                //.set_on_render(hello)
                .default(0)
                .items(&s_arr)
                .interact()
                .unwrap();
        //When selected set that type
        set_value_type_match(s_arr[index],property_name,properties,current_value)
    } else {
        set_value_type_match(s_arr.first().unwrap(),property_name,properties,current_value)
    }
}

fn set_value_type_match(s: &str, property_name: &str, properties: HashMap<String,Value>, current_value: Option<Value>) -> String {
    let l = property_name;
    //Lookup for object within the specific property (using property_name)

    match s {
        NULL => String::from("null"),
        BOOL => {
            //Get prompt and current value (to set as default) from description of schema
            Confirm::new().with_prompt("Do you want to continue?").default(true).interact().unwrap().to_string()
        },
        INT => {
            //Get restrictions/pattern
            //Get prompt from description of schema

            //Look for max and min
            if let Some(x) = properties.get("maximum"){}
            if let Some(x) = properties.get("minimum"){}

            let name = Input::<i32>::new().with_prompt("Your age").interact().unwrap();
            name.to_string()
        },
        NUM => {
            //Get restrictions/pattern
            //Get prompt from description of schema

            //Look for max and min
            if let Some(x) = properties.get("maximum"){}
            if let Some(x) = properties.get("minimum"){}

            let name = Input::<f32>::new().with_prompt("Your height").interact().unwrap();
            name.to_string()
        },
        STRING => {
            //Check property name a run against lookup to check if (path, guid etc.)

            //if backgroundImage -> path
            //if guid -> select via definitions to current profiles guid

            //Contains enum?
            if let Some(x) = properties.get("enum"){}

            let name = Input::<String>::new().with_prompt("Your name").interact().unwrap();
            name
        },
        COLOR => {
            //Check property name a run against lookup to check if (path, guid etc.)

            let transparent_black = CssColor { r: 0, g: 0, b: 0, a: 1.0 };

            let s = &css_color_parser::NAMED_COLORS;


            let o = &css_color_parser::NAMED_COLORS_KEYS;

            o.iter().for_each(|(a)|println!("{:#}",a));

            let name = Input::<String>::new().with_prompt("Your favorite color").interact().unwrap();
            let c = name.parse::<CssColor>()
                .unwrap_or(transparent_black);

            Rgb::from(c.r as f32, c.g as f32, c.b as f32).to_css_hex_string()
        },
        OBJECT => {
            if let Some(rv) = Editor::new().edit("Enter a commit message").unwrap() {
                println!("Your message:");
                println!("{}", rv);
                rv
            } else {
                println!("Abort!");
                let l : HashMap<String,Value>= from_value(current_value.unwrap()).unwrap();
                l.keys().last().unwrap().to_string()
            }
        },
        ARRAY =>
            if let Some(rv) = Editor::new().edit("Enter a commit message").unwrap() {
                println!("Your message:");
                println!("{}", rv);
                rv
            } else {
                println!("Abort!");
                let l : Vec<String> = from_value(current_value.unwrap()).unwrap();
                l.last().unwrap().to_string()
            },
        _ => {
            //Lookup in schema
            //Get "$ref":
            //#/definitions/<DEFINITIONS_NAME>
            String::from("None")
        }
    }
}