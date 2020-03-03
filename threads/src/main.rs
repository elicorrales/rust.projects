use std::thread::spawn;
use std::thread::sleep;
use std::time::Duration;

static mut some_extern_var:u32 = 10;

fn main() {

    //let mut some_extern_var:u32 = 10;
    //let ref_to_ext_var = &some_extern_var;

    let my_thread_using_extern_vars = move ||{
        unsafe {
        sleep(Duration::from_millis(500));
        println!("Thread using extern");
        println!("in thread: {}", some_extern_var);
        //println!("in thread: {}", ref_to_ext_var);
        println!("");
        some_extern_var = 11;
        }
    };

 
    spawn(my_thread_using_extern_vars);

    unsafe {
    println!("In main: {}", some_extern_var);
    }

    sleep(Duration::from_millis(1200));
    unsafe {
    println!("In main: {}", some_extern_var);
    }

    println!("");
    println!("Main thread waiting...");
    println!("");
    sleep(Duration::from_millis(1200));
    println!("");
    println!("Main thread done.");
}
