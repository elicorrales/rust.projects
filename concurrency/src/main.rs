mod book;
use book::*;


fn main() {

    let mut cache:Vec<&Book> = Vec::new();

    query_cache(&cache);

    query_database(&mut cache);

    query_cache(&cache);
}

fn query_cache(cache:&Vec<&Book>) {

    println!("|Query Cache----------------------------------|");
    println!("|                                             |");

    for book in cache {
        println!("| {} {}", book.id, book.author);
    }
    println!("|                                             |");
    println!("-----------------------------------------------");
}


fn query_database(cache:&mut Vec<&Book>) {

    println!("Query Database-------------------------------");
    println!("|                                             |");

    for i in 0..20 {
        let id = BOOKS[i].id;
        let author = BOOKS[i].author;
        println!("| {} {}", id, author);
        cache.push(&BOOKS[i]);
    }
    println!("|                                             |");
    println!("-----------------------------------------------");
}
