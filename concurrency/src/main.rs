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

    let args : Vec<String> = args().collect();

    let mut num_loops:u32 = 0;

    handle_cmdline_args(&args, &mut num_loops);

    let mut cache:HashMap<u32,&Book> = HashMap::new();

    let mut rng = thread_rng();

    for _i in 1..30 {

        let id = rng.gen_range(1,21);

        query_cache(&cache,id);

        query_database(&mut cache,id);
    }

    println!("");

}

fn handle_cmdline_args(args:&Vec<String>, num_loops:&mut u32) {

    println!("{}", args.len());

    match args.len() {
        2 => { 

            println!("{}",args[1]);
            let result = FromStr::from_str(&args[1]);
            match result {
                Ok(loops) => {
                    let l:u32 = loops;
                    println!("{}", l);
                }
                Err(myerr)=> println!("{:?}", myerr)
            }

        }
        _ => usage(args)
    }
}

fn usage(args:&Vec<String>) {
    println!("");
    println!("");
    println!("{} <num loops>", args[0]);
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


fn query_database(cache:&mut HashMap<u32,&Book>,rndid:u32) {

    //mimic delay hitting database...
    thread::sleep(Duration::from_millis(100));
    std::io::stdout().flush().unwrap();
    for i in 0..20 {
        let id = BOOKS[i].id;
        if id == rndid {
            cache.insert(id,&BOOKS[i]);
        }
    }
}
