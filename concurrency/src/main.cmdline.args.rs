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

    println!(""); println!("");

    let args : Vec<String> = args().collect();

    let mut num_loops:u32 = 0;
    let mut delay_ms:u64 = 50;

    if args.len() > 2 {

        let loop_result = <u32 as FromStr>::from_str(&args[1]);
        if loop_result.is_ok() {
            num_loops = loop_result.unwrap();
        }

        let delay_result = <u64 as FromStr>::from_str(&args[2]);
        if delay_result.is_ok() {
            delay_ms = delay_result.unwrap();
        }

    } else {
        usage(&args);
    }

    let mut cache:HashMap<u32,&Book> = HashMap::new();

    let mut rng = thread_rng();

    for _i in 1..num_loops {
        let id = rng.gen_range(1,21);
        query_cache(&cache, id);
        query_database(&mut cache, id, delay_ms);
        if cache.len() >= BOOKS.len() {
            println!(""); println!("");
            println!("Cache is Full.");
            println!("");
            exit(0);
        }
    }

    println!(""); println!("");
    println!("Ran out of loops.");
    println!("");

}


fn usage(args:&Vec<String>) {
    println!("");
    println!("");
    println!("usage: {} <num loops> <db delay ms>", args[0]);
    println!("");
    println!("");
    exit(1);
}

fn query_cache(cache:&HashMap<u32,&Book>,rndid:u32) {

    print!(".");
    let option = cache.get(&rndid);
    match option {
        //Some(book) => { print!("{} ", cache.len()); }
        None       => {/* do nothing */} 
        _          => { print!("{} ", cache.len()); }
    }
}


fn query_database(cache:&mut HashMap<u32,&Book>, rndid:u32, delay:u64) {

    //mimic delay hitting database...
    thread::sleep(Duration::from_millis(delay));
    std::io::stdout().flush().unwrap();
    for i in 0..20 {
        let id = BOOKS[i].id;
        if id == rndid {
            cache.insert(id,&BOOKS[i]);
        }
    }
}
