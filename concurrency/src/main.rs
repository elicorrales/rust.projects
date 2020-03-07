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
use std::time::Instant;

lazy_static! {

    static ref CACHE:Mutex<HashMap<u32,Book>> = Mutex::new(HashMap::new());
    static ref NUM_BOOKS:Mutex<u32> = Mutex::new(0);
    static ref NUM_LOOPS:Mutex<u32> = Mutex::new(0);
    static ref DELAY_MS:Mutex<u64> = Mutex::new(0);
    static ref ID:Mutex<u32> = Mutex::new(0);
    static ref IS_RANDOM:Mutex::<bool> = Mutex::new(true);
}




fn main() {


    handle_cmdline_args();


    let mut rng = thread_rng();

    for i in 0..*NUM_LOOPS.lock().unwrap() {

        let since_start = Instant::now();


        if *IS_RANDOM.lock().unwrap() {
            *ID.lock().unwrap() = rng.gen_range(1,*NUM_BOOKS.lock().unwrap()+1);
        } else {
            if *ID.lock().unwrap() > *NUM_BOOKS.lock().unwrap() {
                *ID.lock().unwrap() = 0;
            } else {
                *ID.lock().unwrap() += 1;
            }
        }

        thread::spawn(|| {
            println!("thread 1");
            query_cache(*ID.lock().unwrap());
        });

        thread::spawn(|| {
            println!("thread 2");
            query_database(*NUM_BOOKS.lock().unwrap() as usize, *ID.lock().unwrap(), *DELAY_MS.lock().unwrap());
        });

        println!("loop:{} time: {:.2?} =======================================",i, since_start.elapsed());
    }

    println!("");

}



fn handle_cmdline_args() {

    let mut num_books:u32 = 1;
    let mut num_loops:u32 = 1;
    let mut delay_ms:u64  = 1;
    let mut is_random:bool = true;

    let args:Vec<String> = args().collect();

    match args.len() {

        1         => {}

        2         => {
                        let result = <u32 as FromStr>::from_str(&args[1]);
                        if result.is_ok() {
                            num_books = result.unwrap();
                        }
                    }

        3         => {
                        let result = <u32 as FromStr>::from_str(&args[1]);
                        if result.is_ok() {
                            num_books = result.unwrap();
                        } else { usage(); }
                        let result = <u32 as FromStr>::from_str(&args[2]);
                        if result.is_ok() {
                            num_loops = result.unwrap();
                        } else { usage(); }
                    }

        4         => {
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
                    }

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

    *NUM_BOOKS.lock().unwrap() = num_books;
    *NUM_LOOPS.lock().unwrap() = num_loops;
    *DELAY_MS.lock().unwrap()  = delay_ms;
    *IS_RANDOM.lock().unwrap() = is_random;

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
    let option = rcache.get(&rndid);
    match option {
        Some(_) => print!("{}", rcache.len()),
        //Some(_) => {}
        None    => {}
    }
    std::io::stdout().flush().unwrap();
    //println!("");
}



fn query_database(num_books:usize, rndid:u32, delay_ms:u64) {

    thread::sleep(Duration::from_millis(delay_ms));
    for i in 0..num_books {
        let id = BOOKS[i].id;
        //print!("i:{} id:{} rnd:{} |", i, id, rndid);
        if id == rndid {
            //let book = &BOOKS[i];
            //let book_copy = BookCopy { id : book.id , author : book.author.to_string() };
            //CACHE.lock().unwrap().insert(id, book_copy);
            CACHE.lock().unwrap().insert(id, BOOKS[i]);
        }
    }
    //println!("");
}


