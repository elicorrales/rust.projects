mod book;

use lazy_static::lazy_static;
use std::sync::Mutex;



lazy_static! {
    static ref cache:Mutex<book::Book> = Mutex::new(book::Book{});
}


fn main() {
}
