#![feature(proc_macro_hygiene, decl_macro)]
use rocket::*;
use std::string::String;

static mut VALUE:u8 = 0;

#[get("/value")]
fn get_value() -> String {

    unsafe {
        println!("{}", VALUE); 
        format!("GET value: {}", VALUE)
    }
}

#[post("/value/<value>")]
fn post_value(value:u8) -> String {

    println!("{}", value); 

    unsafe {
        VALUE = value;
        format!("PUT value {}", VALUE)
    }

}


fn main() {
    ignite()
        .mount("/api",routes![get_value,post_value])
        .launch();
}
