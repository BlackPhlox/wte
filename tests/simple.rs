extern crate wte;

use wte::all::editor::{set_value_types};
use wte::all::diff::{NULL,BOOL,INT,NUM,STRING,COLOR,OBJECT,ARRAY};
use serde_json::Value;
use serde_json::json;

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

fn main(){
    //println!("{:#}",set_value_types(vec![NULL,BOOL,INT,NUM,STRING,COLOR,OBJECT,ARRAY],"", None));
    /*println!("Test - None");
    println!("{:#}",set_value_types(vec!["null"],"", None));
    println!("{:#}",set_value_types(vec!["boolean"],"", None));
    println!("{:#}",set_value_types(vec!["null","boolean"],"", None));

    println!();
    println!("Test - Null");
    println!("{:#}",set_value_types(vec!["null"],"", Some(json![null])));
    println!("{:#}",set_value_types(vec!["boolean"],"", Some(json![false])));
    println!("{:#}",set_value_types(vec!["null","boolean"],"", Some(json![null])));
    */
}

#[cfg(test)]

//Run: cargo test -- --nocapture
//to see print statements

#[test]
fn test_guids() {

}

#[test]
fn test_add() {
    let o : Value = json!([1,2,3]);
    println!("{:#?}",o);
    let o2 = o.as_array();
    println!("{:#?}",o2);
    assert_eq!(o2.is_some(),true);
    //let p : Vec<Value> = o2.unwrap();
    //assert_eq!(to_vec::<i8>(p),vec![1,2,3])
}


