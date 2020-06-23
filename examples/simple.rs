extern crate wte;

use wte::edit::{set_value_types};
use wte::main::{NULL,BOOL,INT,NUM,STRING,COLOR,OBJECT,ARRAY};
//use wte::edit::{;

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
    println!("{:#}",set_value_types(vec![NULL,BOOL,INT,NUM,STRING,COLOR,OBJECT,ARRAY],"", None));
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


