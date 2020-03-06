mod book;
use book::*;
use std::collections::HashMap;
use rand::*;
use std::thread;
use std::time::Duration;
use std::io::Write;
use std::env::args;
use std::process::exit;
use std::str::FromStr;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref CACHE:Mutex<HashMap<u32,&'static Book>> = Mutex::new(HashMap::new());
}

fn main() {


    //let (num_loops, delay_ms) = handle_cmdline_args();

/*
    let mut rng = thread_rng();

    for _i in 1..num_loops {

        let id = rng.gen_range(1,21);

        query_cache(id);

        query_database(id, delay_ms);
    }

    println!("");
*/
}

/*
fn handle_cmdline_args() -> (u32, u64) {

    let mut num_loops = 30;
    let mut delay_ms  = 100;

    let args:Vec<String> = args().collect();

    match args.len() {

        1         => {}

        2         => {
                        let result = <u32 as FromStr>::from_str(&args[1]);
                        if result.is_ok() {
                            num_loops = result.unwrap();
                        }
                    }

        3         => {
                        let result = <u32 as FromStr>::from_str(&args[1]);
                        if result.is_ok() {
                            num_loops = result.unwrap();
                        }
                        let result2 = <u64 as FromStr>::from_str(&args[2]);
                        if result2.is_ok() {
                            delay_ms = result2.unwrap();
                        }
                    }

        _         => usage(&args[0])

    }


    (num_loops, delay_ms)
}
*/


/*
fn usage(prog_name:&String) {
    println!("");
    println!("");
    println!("usage: {} <num_loops> <delay_ms>", prog_name);
    println!("");
    println!("");
    exit(1);
}

fn query_cache(rndid:u32) {

    print!(".");

    let rcache = cache.lock().unwrap();
    let option = rcache.get(&rndid);
    match option {
        Some(_) => print!("{}", rcache.len()),
        None    => {}
    }
    std::io::stdout().flush().unwrap();
}


fn query_database(rndid:u32, delay_ms:u64) {

    thread::sleep(Duration::from_millis(delay_ms));
    for i in 0..20 {
        let id = BOOKS[i].id;
        if id == rndid {
            cache.lock().unwrap().insert(id,&BOOKS[i]);
        }
    }
}
*/
