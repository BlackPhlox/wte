use dialoguer::{Select, Confirm, Input, Editor};
use dialoguer::theme::ColorfulTheme;
use serde_json::{Value, from_value};
use serde_json::json;
use std::collections::HashMap;

fn main(){
    println!("Test - None");
    println!("{:#}",set_value_types(vec!["null"],"", None));
    println!("{:#}",set_value_types(vec!["boolean"],"", None));
    println!("{:#}",set_value_types(vec!["null","boolean"],"", None));
    println!();
    println!("Test - Null");
    println!("{:#}",set_value_types(vec!["null"],"", Some(json![null])));
    println!("{:#}",set_value_types(vec!["boolean"],"", Some(json![false])));
    println!("{:#}",set_value_types(vec!["null","boolean"],"", Some(json![null])));
}

//Change value types:
//Can be an array like [null,string]
/*
null,
boolean, [Y/N]
integer,
number, (float)
string, (sub: path, guid etc.)
color, (hex-code, color wheel?)
object (editor?),
array
*/

fn set_value_types(s_arr: Vec<&str>, property_name: &str, current_value: Option<Value>) -> String{
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
        "null" => String::from("null"),
        "boolean" => {
            //Get prompt and current value (to set as default) from description of schema
            Confirm::new().with_prompt("Do you want to continue?").default(true).interact().unwrap().to_string()
        },
        "integer" => {
            //Get restrictions/pattern
            //Get prompt from description of schema

            //Look for max and min
            if let Some(x) = properties.get("maximum"){}
            if let Some(x) = properties.get("minimum"){}

            let name = Input::<i32>::new().with_prompt("Your name").interact().unwrap();
            name.to_string()
        },
        "number" => {
            //Get restrictions/pattern
            //Get prompt from description of schema

            //Look for max and min
            if let Some(x) = properties.get("maximum"){}
            if let Some(x) = properties.get("minimum"){}

            let name = Input::<f32>::new().with_prompt("Your name").interact().unwrap();
            name.to_string()
        },
        "string" => {
            //Check property name a run against lookup to check if (path, guid etc.)

            //if backgroundImage -> path
            //if guid -> select via definitions to current profiles guid

            //Contains enum?
            if let Some(x) = properties.get("enum"){}

            let name = Input::<String>::new().with_prompt("Your name").interact().unwrap();
            name
        },
        "color" => {
            //Check property name a run against lookup to check if (path, guid etc.)

            let name = Input::<String>::new().with_prompt("Your name").interact().unwrap();
            name
        },
        "object" => {
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
        "array" =>
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