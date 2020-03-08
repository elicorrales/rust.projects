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
//use std::time::Instant;

lazy_static! {

    static ref CACHE:Mutex<HashMap<u32,Book>> = Mutex::new(HashMap::new());
    static ref NUM_BOOKS:Mutex<u32> = Mutex::new(0);
    static ref NUM_LOOPS:Mutex<u32> = Mutex::new(0);
    static ref DELAY_MS:Mutex<u64> = Mutex::new(0);
    static ref IS_RANDOM:Mutex::<bool> = Mutex::new(true);
    static ref RNDID:Mutex<u32> = Mutex::new(0);
}




fn main() {

    let query_cache_closure = || {
        let id = *RNDID.lock().unwrap();
        query_cache(id);
    };

    let query_database_closure = || {
        let num_books = *NUM_BOOKS.lock().unwrap();
        let id = *RNDID.lock().unwrap();
        let delay_ms = *DELAY_MS.lock().unwrap();
        query_database(num_books as usize, id, delay_ms);
    };



    handle_cmdline_args();

    let is_random = *IS_RANDOM.lock().unwrap();
    let num_books = *NUM_BOOKS.lock().unwrap();

    let mut rng = thread_rng();

    for _i in 0..*NUM_LOOPS.lock().unwrap() {

        //let since_start = Instant::now();


        let mut id = 0;
        if is_random {
            id = rng.gen_range(1,num_books+1);
        } else {
            if id > num_books {
                id = 0;
            } else {
                id += 1;
            }
        }
        *RNDID.lock().unwrap() = id;

        thread::spawn(query_cache_closure);

        thread::spawn(query_database_closure);

        //println!("loop:{} time: {:.2?} =======================================",i, since_start.elapsed());
    }

    println!("");
    println!("");
    println!("Main is waiting....");
    thread::sleep(Duration::from_millis(1000));
    println!("");
    println!("");
}



fn handle_cmdline_args() {

    let mut num_books:u32 = 1;
    let mut num_loops:u32 = 1;
    let mut delay_ms:u64  = 1;
    let mut is_random:bool = true;

    let args:Vec<String> = args().collect();

    match args.len() {

        5         => {
                        let result = <u32 as FromStr>::from_str(&args[1]);
                        if result.is_ok() {
                            num_books = result.unwrap();
                        } else { usage(); }
                        let result = <u32 as FromStr>::from_str(&args[2]);
                        if result.is_ok() {
                            num_loops = result.unwrap();
                        } else { usage(); }

                        let result2 = <u64 as FromStr>::from_str(&args[3]);
                        if result2.is_ok() {
                            delay_ms = result2.unwrap();
                        } else { usage(); }


                        if &args[4] == "n" {
                            is_random = false;
                        }
                    }
        _         => usage()

    }

    if num_books < 1 {
        println!("");
        println!("Num books must be at least 1");
        usage();
    }

    if (num_books as usize) > BOOKS.len() {
        println!("");
        println!("Num books must be no more than {}.", BOOKS.len());
        usage();
    }

    *NUM_BOOKS.lock().unwrap() = num_books;
    *NUM_LOOPS.lock().unwrap() = num_loops;
    *DELAY_MS.lock().unwrap()  = delay_ms;
    *IS_RANDOM.lock().unwrap() = is_random;

    println!("Running with nbks: {} , nloops: {}, ms: {}, isrnd: {:?}",
        num_books, num_loops, delay_ms, is_random);
}



fn usage() {
    let args:Vec<String> = args().collect();
    println!("");
    println!("");
    println!("usage: {} <num_books, <num_loops> <delay_ms> <is_random>", &args[0]);
    println!("");
    println!("");
    exit(1);
}




fn query_cache(rndid:u32) {

    print!(".");

    let rcache = CACHE.lock().unwrap();
    let len    = rcache.len();
    let option = rcache.get(&rndid);
    match option {
        Some(_) => print!("{} ", len),
        None    => {}
    }
    std::io::stdout().flush().unwrap();
}



fn query_database(num_books:usize, rndid:u32, delay_ms:u64) {
    thread::sleep(Duration::from_millis(delay_ms));
    for i in 0..num_books {
        let id = BOOKS[i].id;
        if id == rndid {
            (*CACHE.lock().unwrap()).insert(id, BOOKS[i]);
        }
    }
}


