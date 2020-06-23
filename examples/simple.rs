extern crate wte;

#[path = "../src/all/diff.rs"]
mod diff;

#[path = "../src/all/edit.rs"]
mod edit;

use edit::{set_value_types};
use diff::{NULL,BOOL,INT,NUM,STRING,COLOR,OBJECT,ARRAY};

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


