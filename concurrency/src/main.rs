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

    static ref CACHE:Mutex<HashMap<u32,Book>> = Mutex::new(HashMap::new());
    static ref NUM_BOOKS:Mutex<u32> = Mutex::new(0);
    static ref DELAY_MS:Mutex<u64> = Mutex::new(0);
}


fn main() {


    let (num_books, num_loops, delay_ms, is_random) = handle_cmdline_args();


    let mut rng = thread_rng();

    let mut id = 0;
    for _i in 1..num_loops {

        if is_random {
            id = rng.gen_range(1,num_books+1);
        } else {
            if id > num_books {
                id = 0;
            } else {
                id += 1;
            }
        }

        query_cache(id);

        query_database(num_books as usize, id, delay_ms);
    }

    println!("");

}



fn handle_cmdline_args() -> (u32, u32, u64, bool) {

    let mut num_books = 5;
    let mut num_loops = 10;
    let mut delay_ms  = 1;
    let mut is_random = true;

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
    *DELAY_MS.lock().unwrap()  = delay_ms;

    (num_books, num_loops, delay_ms, is_random)
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
        None    => {}
    }
    std::io::stdout().flush().unwrap();
}



fn query_database(num_books:usize, rndid:u32, delay_ms:u64) {

    thread::sleep(Duration::from_millis(delay_ms));
    for i in 1..=num_books {
        println!("num_books {} rndid:{}", num_books, rndid);
        let id = BOOKS[i].id;
        if id == rndid {
            //let book = &BOOKS[i];
            //let book_copy = BookCopy { id : book.id , author : book.author.to_string() };
            //CACHE.lock().unwrap().insert(id, book_copy);
            CACHE.lock().unwrap().insert(id, BOOKS[i]);
        }
    }
}

