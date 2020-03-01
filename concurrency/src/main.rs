mod book;
use book::*;
use std::collections::HashMap;
use rand::*;

fn main() {

    let mut cache:HashMap<u32,&Book> = HashMap::new();

    let mut rng = thread_rng();

    for i in 1..30 {

        let id = rng.gen_range(1,21);

        query_cache(&cache,id);

        query_database(&mut cache,id);
    }


}

fn query_cache(cache:&HashMap<u32,&Book>,rndid:u32) {

    let option = cache.get(&rndid);
    match option {
        Some(book) => { println!("id: {} len:{} ", rndid, cache.len()); }
        None       => println!("no book")
    }
    println!("-----------------------------------------------");
}


fn query_database(cache:&mut HashMap<u32,&Book>,rndid:u32) {

    for i in 0..20 {
        let id = BOOKS[i].id;
        if id == rndid {
            let author = BOOKS[i].author;
            cache.insert(id,&BOOKS[i]);
        }
    }
}
