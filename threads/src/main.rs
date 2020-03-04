use std::thread::spawn;
use std::thread::sleep;
use std::time::Duration;
use lazy_static::lazy_static;
use std::sync::Mutex;


lazy_static! {
    static ref SOME_EXTERN_VAR:Mutex<u32> = Mutex::new(10);
}


fn main() {

    let my_thread_using_extern_vars = ||{
        sleep(Duration::from_millis(500));
        println!("Thread using extern");
        println!("in thread: {:?}", SOME_EXTERN_VAR.lock().unwrap());
        let mut ext = SOME_EXTERN_VAR.lock().unwrap();
        println!("in thread: {:?}", ext);
        *ext = 11;
        println!("in thread: {:?}", ext);
        println!("");
    };

 
    spawn(my_thread_using_extern_vars);

    //this insures that thread grabs extern first and modifies it.
    //if this line is commented, main might grab ext first 
    sleep(Duration::from_millis(1200));

    let ext = SOME_EXTERN_VAR.lock().unwrap();
    println!("In main: {}", ext);

    sleep(Duration::from_millis(1200));

    println!("In main: {}", ext);

    println!("");
    println!("Main thread waiting...");
    println!("");
    sleep(Duration::from_millis(1200));
    println!("");
    println!("Main thread done.");
}

