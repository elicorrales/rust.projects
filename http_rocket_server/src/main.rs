#![feature(proc_macro_hygiene, decl_macro)]

//#[macro_use]
//extern crate lazy_static;

use rocket::*;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref VALUE:Mutex<u8> = Mutex::new(0);
}


#[post("/value/<val>")]
fn post_value(val:u8) -> String {
    {
        println!("client 1 has hit value");
        let mut v = VALUE.lock().unwrap();
        println!("client 1 has locked value");
        *v = val;

        thread::sleep(Duration::from_millis(2000));
        let val2 = *v;
        println!("client 1 is releasing lock");
        if val != val2 {
            return format!("ERROR");
        }

        println!("");
        format!("OK")
    }
}


#[post("/value2/<val>")]
fn post_value2(val:u8) -> String {
    {
        println!("client 2 has hit value2");
        let mut v = VALUE.lock().unwrap();
        println!("client 2 has locked value2");
        *v = val;

        thread::sleep(Duration::from_millis(2000));
        let val2 = *v;
        println!("client 2 is releasing lock");
        if val != val2 {
            return format!("ERROR");
        }

        println!("");
        format!("OK")
    }
}




fn main() {
    ignite()
        .mount("/api",routes![post_value, post_value2])
        .launch();
}

