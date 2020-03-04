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

fn main() {
    let mut num_loops = 30;
    let mut delay_ms  = 100;

    let args:Vec<String> = args().collect();

    match args.len() {

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
                        let result2 = <u32 as FromStr>::from_str(&args[2]);
                        if result2.is_ok() {
                            delay_ms = result2.unwrap().into();
                        }
                    }

        4|5|6|7|8 => { println!("Not Implemented."); usage(&args[0]); }

        _         => usage(&args[0])

    }

    let mut cache:HashMap<u32,&Book> = HashMap::new();

    let mut rng = thread_rng();

    for _i in 1..num_loops {

        let id = rng.gen_range(1,21);

        query_cache(&cache,id);

        query_database(&mut cache,id, delay_ms);
    }

    println!("");

}

fn usage(prog_name:&String) {
    println!("");
    println!("");
    println!("usage: {} <num_loops> <delay_ms>", prog_name);
    println!("");
    println!("");
    exit(1);
}

fn query_cache(cache:&HashMap<u32,&Book>,rndid:u32) {

    print!(".");

    let option = cache.get(&rndid);
    match option {
        Some(_) => print!("{}", cache.len()),
        None    => {/*print!(".")*/}
    }

    std::io::stdout().flush().unwrap();
}


fn query_database(cache:&mut HashMap<u32,&Book>, rndid:u32, delay_ms:u64) {

    thread::sleep(Duration::from_millis(delay_ms));
    for i in 0..20 {
        let id = BOOKS[i].id;
        if id == rndid {
            cache.insert(id,&BOOKS[i]);
        }
    }
}

/*
use std::thread;
use std::time::Duration;
use std::io::Write;
    thread::sleep(Duration::from_millis(100));
    //mimic delay hitting database...
*/
