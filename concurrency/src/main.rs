mod book;
use book::*;
use std::collections::HashMap;
use rand::*;
use std::thread;
use std::time::{ Duration, Instant };
use std::io::Write;
use std::env::args;
use std::process::exit;
use std::str::FromStr;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref CACHE:Mutex<HashMap<u32, BookCopy>> = Mutex::new(HashMap::new());
}

fn main() {

    let since_start = Instant::now();

    let mut rng = thread_rng();

    let (num_books, num_loops, delay_ms, do_random, do_threaded) = command_line_handler();

    let mut id = 0;
    for _i in 0..num_loops {

        if CACHE.lock().unwrap().len() >= BOOKS.len() {
            exit_when_full(since_start);
        }

        if do_random {
            id = rng.gen_range(1,num_books+1);
        } else {
            if id >= num_books {
                id = 1;
            } else {
                id += 1;
            }
        }

        if do_threaded {
            thread::spawn(move || { query_cache(id); } );
            thread::spawn(move || { query_database(num_books as usize, id, delay_ms); });
        } else {
            query_cache(id);
            query_database(num_books as usize, id, delay_ms);
        }


    }

    println!("");
    println!("");
    println!("Main is waiting....");
    if do_threaded {
        thread::sleep(Duration::from_millis(1000));
    } else {
        thread::sleep(Duration::from_millis((num_loops as u64) * delay_ms));
    }
    println!("Main is DONE, we may not have finished filling cache with all books.");
    println!("");
    println!("Elapsed time: {:.2?}", since_start.elapsed());
    println!("");
}


fn command_line_handler() -> (u32, u32, u64, bool, bool) {
    let mut num_books:u32 = 1;
    let mut num_loops:u32 = 1;
    let mut delay_ms:u64  = 1;
    let mut do_random:bool = false;
    let mut do_threaded:bool = false;

    let args:Vec<String> = args().collect();

    match args.len() {

        6         => {
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


                        match (&args[4]).as_str() {
                            "n" => do_random = false,
                            "y" => do_random = true,
                            _   => usage()
                        }

                        match (&args[5]).as_str() {
                            "n" => do_threaded = false,
                            "y" => do_threaded = true,
                            _   => usage()
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

    println!("Running with nbks: {} , nloops: {}, ms: {}, rnd: {:?}, thrd:{:?}",
        num_books, num_loops, delay_ms, do_random, do_threaded);

    (num_books, num_loops, delay_ms, do_random, do_threaded)

}



fn usage() {
    let args:Vec<String> = args().collect();
    println!("");
    println!("");
    println!("usage: {} <num_books>, <num_loops> <delay_ms> <do_random> <do_threaded>", &args[0]);
    println!("");
    println!("");
    //panic!();
    exit(1);
}

fn exit_when_full(since_start:Instant) {
    println!("");
    println!("");
    println!("Cache is full, exiting early.");
    println!("");
    println!("Elapsed time: {:.2?}", since_start.elapsed());
    println!("");
    exit(0);
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
            let book = BookCopy { id: BOOKS[i].id, author: BOOKS[i].author.to_string() };
            CACHE.lock().unwrap().insert(id, book);
            break;
        }
    }
}


