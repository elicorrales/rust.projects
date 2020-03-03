use std::thread::spawn;
use std::thread::sleep;
use std::time::Duration;

fn main() {

    let my_thread = |num|{
        sleep(Duration::from_millis(1000));
        println!("Thread {}", num);
        println!("");
    };

    println!("");
    println!("Main thread starts other threads..");

    my_thread(1);
    my_thread(2);

    spawn(my_thread(1));
    spawn(my_thread(2));


    println!("");
    println!("Main thread waiting...");
    println!("");
    sleep(Duration::from_millis(1200));
    println!("");
    println!("Main thread done.");
}
